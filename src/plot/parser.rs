use crate::intrinsics::*;
use crate::{error, sim};

/// This parses the spice output and return the labels for each series and the points to plot the
/// graphs. (x_label, y_labels, series)
pub fn parse_spice_output(
    probes: &sim::Probes,
    output: &str,
) -> (String, Vec<String>, Vec<Vec<Point>>) {
    let mut is_header_found = false;
    let mut x_label = "".to_string();
    let mut y_labels = Vec::new();
    let mut series = Vec::new();
    output.lines().for_each(|line| {
        let cols = line.split_whitespace().collect::<Vec<&str>>();
        if !is_header_found && cols.len() > 0 {
            if cols[0] == "Index" {
                x_label = cols[1].to_string();
                for idx in 2..cols.len() {
                    series.push(Vec::new());
                    y_labels.push(probes[idx - 2].name.clone());
                }
                is_header_found = true;
            }
        } else if is_header_found {
            // We check if we have at least a number for the time and a value match with that time.
            if cols.len() > 1 && cols[0].parse::<usize>().is_ok() {
                if let Ok(x) = cols[1].parse::<f64>() {
                    for idx in 2..cols.len() {
                        if let Ok(y) = cols[idx].parse::<f64>() {
                            series[idx - 2].push(Point::new(x, y));
                        }
                    }
                }
            }
        }
    });
    if !is_header_found {
        let error: Box<dyn std::error::Error> = Box::new(error::Sim::SpiceNoData);
        error::show(error);
    }
    (x_label, y_labels, series)
}
