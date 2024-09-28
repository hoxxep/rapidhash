use plotters::prelude::*;
use serde::Deserialize;

/// Build charts for the README using criterion benchmarking results.
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grey = RGBColor(200, 200, 200);
    let hash_settings = [
        ("rapidhash", BLUE),
        ("default", BLACK),
        ("fxhash", RED),
        ("gxhash", MAGENTA),
        ("wyhash", CYAN),
        ("ahash", grey),
        ("t1ha", grey),
        ("xxhash", grey),
        ("metrohash", grey),
        ("seahash", grey),
    ];

    let hash_functions = hash_settings.iter().map(|(name, _)| *name).collect::<Vec<_>>();

    let sizes = [2, 8, 16, 64, 256, 1024, 4096];

    let mut latency_data = vec![];
    let mut throughput_data = vec![];

    for hash_function in hash_functions.iter() {
        let mut latency_row = vec![];
        let mut throughput_row = vec![];
        for size in sizes.iter() {
            let mut measurements: Vec<_> = std::fs::read_dir(format!("target/criterion/data/main/hash_{}/str_{}", hash_function, size))?
                .map(|p| p.unwrap().file_name().into_string().unwrap())
                .filter(|p| p.starts_with("measurement"))
                .collect();
            measurements.sort();
            let last_measurement = measurements.last().unwrap();
            let file = std::fs::File::open(format!("target/criterion/data/main/hash_{}/str_{}/{}", hash_function, size, last_measurement))?;

            let measurement: CriterionMeasurement = serde_cbor::from_reader(file)?;
            // println!("{measurement:?}");
            let latency = measurement.estimates.mean.point_estimate;
            latency_row.push(latency as f32);
            let throughput = (1_000_000_000f64 / latency) * (*size as f64) / 1_000_000_000f64;  // GB/s
            throughput_row.push(throughput as f32);
        }
        latency_data.push(latency_row);
        throughput_data.push(throughput_row);
    }

    let root_area = SVGBackend::new("charts.svg", (1024, 768)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let graph_areas = root_area.split_evenly((2, 2));

    {  // latency chart
        let mut cc = ChartBuilder::on(&graph_areas[0])
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("latency", ("sans-serif", 30))
            .build_cartesian_2d((2..4096).log_scale(), (1f32..1_000.).log_scale())?;

        cc.configure_mesh()
            // .x_labels(20)
            // .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{v:.0}"))
            .y_label_formatter(&|v| format!("{v:.0}"))
            .draw()?;

        for (i, (hash_function, color)) in hash_settings.iter().enumerate() {
            cc.draw_series(LineSeries::new(sizes.iter().zip(latency_data[i].iter()).map(|(x, y)| (*x, *y)), color))?
                .label(*hash_function)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
        }

        cc.configure_series_labels().border_style(BLACK).draw()?;
    }

    {  // throughput chart
        let mut cc = ChartBuilder::on(&graph_areas[1])
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption("throughput", ("sans-serif", 30))
            .build_cartesian_2d((2..4096).log_scale(), (0.1f32..80.).log_scale())?;

        cc.configure_mesh()
            // .x_labels(20)
            // .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{v:.0}"))
            .y_label_formatter(&|v| format!("{v:.0}"))
            .draw()?;

        for (i, (hash_function, color)) in hash_settings.iter().enumerate() {
            cc.draw_series(LineSeries::new(sizes.iter().zip(throughput_data[i].iter()).map(|(x, y)| (*x, *y)), color))?
                .label(*hash_function)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color.clone()));
        }

        cc.configure_series_labels().border_style(BLACK).draw()?;
    }

    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", "charts.svg");

    Ok(())
}

#[derive(Debug, Deserialize)]
struct CriterionMean {
    point_estimate: f64,
}

#[derive(Debug, Deserialize)]
struct CriterionEstimates {
    mean: CriterionMean,
}

#[derive(Debug, Deserialize)]
struct CriterionMeasurement {
    estimates: CriterionEstimates
}
