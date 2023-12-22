
use plotters::prelude::IntoDrawingArea;

use crate::{holdings::*, util::TimeWindow};

const OUTPUT_FPATH: &str = "static/img/history.svg";



pub fn create_history_plot(
    history: holding::Records,
    window: TimeWindow,
    time_markers: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = plotters::backend::SVGBackend::new(
        OUTPUT_FPATH,
        (1024, 256)
    ).into_drawing_area();

    root.fill(&plotters::style::colors::WHITE).unwrap();

    let history = history.window(window.clone());

    let mut chart = plotters::chart::ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .caption("Investment History", ("sans-serif", 40))
        .build_cartesian_2d(
            window.start..window.end,
            history.minimum()..history.maximum(),
        )?;

    chart.configure_mesh()
        .x_labels(history.len())
        .max_light_lines(4)
        .y_desc("Net Value")
        .draw()?;

    chart.draw_series(
        plotters::series::LineSeries::new(
            history.0.iter().map(|record| {
                let x = record.time;
                let y: f64 = record.net_value();
                return (x, y);
            }),
            plotters::style::colors::BLACK,
        ).point_size(5),
    )?;

    chart.draw_series(
        history.0.iter().map(|record| {
            return plotters::element::Circle::new(
                (record.time, record.net_value()),
                3,
                &plotters::style::colors::BLACK,
            );
        })
    )?;

    root.present()?;
    return Ok(());
}

pub fn chart_from_options(
    accounts: Vec<String>,
    start: Option<String>,
    end: Option<String>,
    markers: Option<u32>,
) -> Result<(), Box<dyn std::error::Error>>{

    let history = crate::
        holdings::
        holding::
        Records::
        from_file("timeline.ron")
        .unwrap();

    let start = match start {
        Some(_) => todo!(),
        None => history.start(),
    };
    let end = match end {
        Some(_) => todo!(),
        None => history.end(),
    };

    let history = history.window(TimeWindow { start, end });
    let history = history.filter_accounts(accounts);

    let markers = match markers {
        Some(num) => num,
        None => 10u32
    };

    create_history_plot(
        history,
        TimeWindow { start, end },
        markers
    )?;

    return Ok(());
    
}


