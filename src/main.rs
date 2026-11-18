use anyhow::{Result, anyhow};
use biblatex::{Bibliography, Date, DateValue, PermissiveType};
use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    coord::ranged1d::IntoSegmentedCoord,
    drawing::IntoDrawingArea,
    series::Histogram,
    style::{Color, RED, WHITE},
};
use std::{env::args, fs::read_to_string};

// TODO: Better name
const OUT_FILE_NAME: &str = "test.png";

// TODO: Think about this
fn get_year(d: Date) -> Result<u32> {
    match d.value {
        DateValue::At(y) => Ok(y.year as u32),
        DateValue::Before(y) => Err(anyhow!(format!("Before {}", y.year))),
        DateValue::After(y) => Err(anyhow!(format!("After {}", y.year))),
        DateValue::Between(x, y) => Err(anyhow!(format!("Between {} {}", x.year, y.year))),
    }
}

// TODO: Improve looks
fn plot(data: &[u32]) -> Result<()> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Publication year", ("sans-serif", 50.0))
        // TODO: compute max and min of the data
        .build_cartesian_2d((1965u32..2027u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn main() -> Result<()> {
    let src_file = args().nth(1).expect("Missing source file");
    println!("{src_file}");
    let src = read_to_string(src_file)?;
    let dates = Bibliography::parse(src.as_str())?
        .iter()
        .filter_map(|e| {
            e.date().ok().map(|d| match d {
                PermissiveType::Typed(d) => Some(get_year(d).unwrap()),
                PermissiveType::Chunks(_) => panic!("Idk how to handle this"),
            })
        })
        .map(|e| e.expect("impossible"))
        .collect::<Vec<u32>>();

    plot(&dates)
}
