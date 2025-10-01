use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;

/// Finds the next segment based on `maze` and the taken `path`.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
pub fn next(maze: &Maze, path: &Path) -> Segment {
    let mut min_segment = Segment::new();

    for i in 0..path.size() {
        let current = path.segment(i);

        for dir in [
            Relative::NORTH,
            Relative::EAST,
            Relative::SOUTH,
            Relative::WEST,
        ] {
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

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use crate::path::Path;
    use crate::pathfinder;
    use crate::vec::Veci;

    #[test]
    fn next() {
        let maze = Maze::new();
        let mut path = Path::new();

        loop {
            let next = pathfinder::next(&maze, &path);
            path.append(next);

            if next.distance == 0 {
                break;
            }
        }

        assert_eq!(15, path.size());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).pos());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).pos());
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).pos());
        assert_eq!(Veci { x: 3, y: 0 }, path.segment(3).pos());
        assert_eq!(Veci { x: 4, y: 0 }, path.segment(4).pos());
        assert_eq!(Veci { x: 5, y: 0 }, path.segment(5).pos());
        assert_eq!(Veci { x: 6, y: 0 }, path.segment(6).pos());
        assert_eq!(Veci { x: 7, y: 0 }, path.segment(7).pos());
        assert_eq!(Veci { x: 7, y: 1 }, path.segment(8).pos());
        assert_eq!(Veci { x: 7, y: 2 }, path.segment(9).pos());
        assert_eq!(Veci { x: 7, y: 3 }, path.segment(10).pos());
        assert_eq!(Veci { x: 7, y: 4 }, path.segment(11).pos());
        assert_eq!(Veci { x: 7, y: 5 }, path.segment(12).pos());
        assert_eq!(Veci { x: 7, y: 6 }, path.segment(13).pos());
        assert_eq!(Veci { x: 7, y: 7 }, path.segment(14).pos());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(15).pos());
    }
}
