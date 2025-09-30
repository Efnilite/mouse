use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;

pub fn next(maze: &Maze, path: &Path) -> Segment {
    let mut min_segment = Segment::new();

    for i in 0..path.size() {
        let current = path.segment(i);

        for dir in [Relative::NORTH, Relative::WEST, Relative::EAST, Relative::SOUTH] {
            let segment = current.relative(maze, dir);

            if segment.is_err() {
                continue;
            }

            let segment = segment.unwrap();

            if segment.distance < min_segment.distance {
                min_segment = segment;
            }
        }
    }

    min_segment
}