//! Behavioural tests for rapidhash seeding on wasm32, run through wasmtime.
//!
//! wasm32 has full atomics but no ASLR or threads, so rapidhash's seeding entropy depends
//! entirely on whether the `getrandom` feature is enabled and the host provides entropy.
//! These tests pin down both configurations:
//!
//! - **Without `getrandom`** (`wasm32-unknown-unknown`): seeding is fully deterministic.
//!   Within an instance, per-map `RandomState` seeds must still be unique (the seed counter
//!   works) and `GlobalState` must initialize once and stay stable; across fresh instances,
//!   all outputs are identical because there is no entropy source. This documents that the
//!   default configuration has NO HashDoS resistance on wasm.
//! - **With `getrandom`** (`wasm32-wasip1`, where the WASI host supplies entropy via
//!   `random_get`): outputs must DIFFER across fresh instances, proving the entropy reaches
//!   both the global secrets and the per-map seeds.
//!
//! Browser/node environments (`wasm32-unknown-unknown` + getrandom's `wasm_js` backend) can't
//! be tested under wasmtime as they need a JS host, but they share the same getrandom code
//! path as the WASI test. Enabling `getrandom` on wasm32-unknown-unknown without the backend
//! flag is a compile error (getrandom refuses to build), so it cannot silently regress to the
//! deterministic behaviour.

use std::path::PathBuf;
use std::process::Command;
use wasmtime::*;

/// Build the wasm module for the given target, returning `None` (test skipped) when the
/// required rustup target isn't installed.
fn build_wasm(target: &str, features: &[&str]) -> Option<PathBuf> {
    let installed = Command::new("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .ok()?;
    if !String::from_utf8_lossy(&installed.stdout)
        .lines()
        .any(|line| line.trim() == target)
    {
        return None;
    }

    let mut args = vec![
        "build",
        "--release",
        "--package", "rapidhash-bench-wasm",
        "--target", target,
    ];
    for feature in features {
        args.extend_from_slice(&["--features", feature]);
    }

    let status = Command::new("cargo")
        .args(&args)
        .env("RUSTFLAGS", "") // explicitly clear any bench flags
        .status()
        .expect("Failed to run cargo build for wasm target");
    assert!(status.success(), "Failed to compile for the {target} target");

    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../target"));
    let path = target_dir.join(target).join("release/rapidhash_bench_wasm.wasm");
    assert!(path.exists(), "Expected output wasm file not found at {}", path.display());
    Some(path)
}

/// A fresh module instance: fresh linear memory, so all of rapidhash's statics are reset.
struct WasmInstance<T: 'static> {
    store: Store<T>,
    random_state: TypedFunc<(), u64>,
    global_state: TypedFunc<(), u64>,
}

impl<T: 'static> WasmInstance<T> {
    fn new(linker: &Linker<T>, module: &Module, data: T) -> Self {
        let mut store = Store::new(linker.engine(), data);
        let instance = linker.instantiate(&mut store, module).unwrap();

        // wasip1 cdylibs are reactor-style modules that may export an initializer.
        if let Some(initialize) = instance.get_func(&mut store, "_initialize") {
            initialize.call(&mut store, &[], &mut []).unwrap();
        }

        let random_state = instance.get_typed_func::<(), u64>(&mut store, "test_wasm_random_state").unwrap();
        let global_state = instance.get_typed_func::<(), u64>(&mut store, "test_wasm_global_state").unwrap();
        Self { store, random_state, global_state }
    }

    fn random_state(&mut self) -> u64 {
        self.random_state.call(&mut self.store, ()).unwrap()
    }

    fn global_state(&mut self) -> u64 {
        self.global_state.call(&mut self.store, ()).unwrap()
    }
}

#[test]
fn test_wasm_seeding_without_getrandom_is_deterministic() {
    let Some(path) = build_wasm("wasm32-unknown-unknown", &[]) else {
        eprintln!("skipping: wasm32-unknown-unknown target not installed (rustup target add wasm32-unknown-unknown)");
        return;
    };

    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).unwrap();
    let linker: Linker<()> = Linker::new(&engine);

    let mut a = WasmInstance::new(&linker, &module, ());

    // The seed counter must produce a distinct seed per RandomState, even without entropy.
    let a_random1 = a.random_state();
    let a_random2 = a.random_state();
    assert_ne!(a_random1, a_random2, "RandomState instances should have unique seeds within a wasm instance");

    // GlobalState initializes once and every subsequent call must agree.
    let a_global1 = a.global_state();
    let a_global2 = a.global_state();
    assert_eq!(a_global1, a_global2, "GlobalState should be stable within a wasm instance");

    // A fresh instance resets all statics; with no entropy source on wasm32-unknown-unknown
    // the outputs are identical across instances. This documents the platform's limitation:
    // there is NO HashDoS resistance in this configuration.
    let mut b = WasmInstance::new(&linker, &module, ());
    assert_eq!(a_random1, b.random_state(), "wasm32 seeding without getrandom is expected to be deterministic across instances");
    assert_eq!(a_global1, b.global_state(), "wasm32 global secrets without getrandom are expected to be deterministic across instances");
}

#[test]
fn test_wasm_seeding_with_getrandom_is_randomized() {
    use wasmtime_wasi::p1::WasiP1Ctx;

    let Some(path) = build_wasm("wasm32-wasip1", &["getrandom"]) else {
        eprintln!("skipping: wasm32-wasip1 target not installed (rustup target add wasm32-wasip1)");
        return;
    };

    let engine = Engine::default();
    let module = Module::from_file(&engine, &path).unwrap();
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |ctx| ctx).unwrap();

    let wasi_ctx = || wasmtime_wasi::WasiCtxBuilder::new().build_p1();

    let mut a = WasmInstance::new(&linker, &module, wasi_ctx());

    // The uniqueness and stability guarantees hold as before.
    let a_random1 = a.random_state();
    let a_random2 = a.random_state();
    assert_ne!(a_random1, a_random2, "RandomState instances should have unique seeds within a wasm instance");

    let a_global1 = a.global_state();
    let a_global2 = a.global_state();
    assert_eq!(a_global1, a_global2, "GlobalState should be stable within a wasm instance");

    // A fresh instance re-seeds from host entropy: with getrandom enabled, both the global
    // secrets and the per-map seeds must now differ between instances ("boots").
    let mut b = WasmInstance::new(&linker, &module, wasi_ctx());
    assert_ne!(a_random1, b.random_state(), "wasm32 seeding with getrandom should differ across instances");
    assert_ne!(a_global1, b.global_state(), "wasm32 global secrets with getrandom should differ across instances");
}
