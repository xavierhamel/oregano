use crate::intrinsics::*;

/// This parses the spice output and return the labels for each series and the points to plot the
/// graphs. (x_label, y_labels, series)
pub fn parse_spice_output(output: &str) -> (String, Vec<String>, Vec<Vec<Point>>) {
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
                    y_labels.push(cols[idx].to_string());
                }
                is_header_found = true;
            }
        } else if is_header_found {
            // If we have as many columns as series and the first one can be represented as a
            // usize, we have a data line.
            if cols.len() == series.len() + 2 && cols[0].parse::<usize>().is_ok() {
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
    (x_label, y_labels, series)
}
