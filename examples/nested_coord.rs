use plotters::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        BitMapBackend::new("plotters-doc-data/nested_coord.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Nested Coord", ("sans-serif", 50.0).into_font())
        .build_ranged(
            ["Linear", "Quadratic"].nested_coord(|_| 0.0..10.0),
            0.0..10.0,
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..10)
            .map(|x| x as f64 / 1.0)
            .map(|x| ((&"Linear", Some(x)), x)),
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (0..10)
            .map(|x| x as f64 / 1.0)
            .map(|x| ((&"Quadratic", Some(x)), x * x / 10.0)),
        &RED,
    ))?;

    Ok(())
}
