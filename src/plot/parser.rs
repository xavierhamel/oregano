use crate::intrinsics::*;

pub fn parser(input: String) -> Vec<Point> {
    let mut points = Vec::new();
    input.lines().for_each(|line| {
        let cols = line.split(" ");
        // If we have 3 columns and the first one can be represented as a usize, we have a data
        // line.
        if cols.len() == 3 && cols[0].parse::<usize>().is_ok() {
            if let (Ok(x), Ok(y)) = (cols[1].parse::<f64>(), cols[2].parse::<f64>()) {
                points.push(Point::new(x, y));
            }
        }
    })
}
