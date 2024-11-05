// Define a Point struct with x, y coordinates and a wkid.
pub struct Point {
    x: f64,
    y: f64,
    wkid: i64
}

/**
Creates a vector of `Point` instances.

# Parameters
- `count`: The number of points to create.
- `default_point`: An optional `Point` to use as the default value for each point.

# Returns
A vector containing `count` points, each initialized to the values of `default_point` if provided,
or to `(0.0, 0.0, 4326)` if not.
*/
pub fn create_points(count: usize, default_point: Option<Point>) -> Vec<Point> {
    let default_point = default_point.unwrap_or(Point { x: 0.0, y: 0.0, wkid: 4326 });
    let mut points = Vec::with_capacity(count);
    for _ in 0..count {
        // Each point is a new instance, avoiding shared references
        points.push(Point { x: default_point.x, y: default_point.y, wkid: default_point.wkid });
    }
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_memory() {
        let mut sample_points = create_points(5, None);

        // Modify one point
        sample_points[0].x = 1.0;
        sample_points[0].y = 1.0;

        // Modify another point
        sample_points[1].x = 2.0;
        sample_points[1].y = 2.0;

        // Assert the coordinates of the first point
        assert!(sample_points[0].x == 1.0 && sample_points[0].y == 1.0, "The first point was modified!");

        // Check the points
        for point in &sample_points {
            println!("Point(x={}, y={})", point.x, point.y);
        }
    }
}
