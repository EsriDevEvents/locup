use std::sync::{Arc, Mutex};
use std::thread;

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
    fn run_concurrency() {
        let count = 5;
        let sample_points = create_points(count, None);

        // Wrap the points in an Arc and Mutex to share safely between threads
        let points_list = Arc::new(Mutex::new(sample_points));

        // Create a vector to hold the thread handles
        let mut handles = vec![];

        for index in 0..5 {
            // Clone the Arc to share ownership with the new thread
            let points_list = Arc::clone(&points_list);
    
            // Spawn a new thread
            let handle = thread::spawn(move || {
                let mut points = points_list.lock().unwrap();
                points[index].x = index as f64 + 1.0;
                points[index].y = index as f64 + 1.0;
            });
    
            // Store the thread handle
            handles.push(handle);
        }
    
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
    
        // Print the points
        let points = points_list.lock().unwrap();
        for point in points.iter() {
            println!("Point(x={}, y={})", point.x, point.y);
        }

        // Assert the coordinates of the first point
        assert!(points[0].x == 1.0 && points[0].y == 1.0, "The first point was modified!");
        // Explain borrowing and ownership
        // https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
        //assert!(sample_points[0].x == 1.0 && sample_points[0].y == 1.0, "The first point was modified!");
    }
}
