use afl::fuzz;

fn main() {
    fuzz!(|data: &[u8]| {
        // fuzzed code goes here
        let _ = rapidhash::rapidhash(data);
    });
}
