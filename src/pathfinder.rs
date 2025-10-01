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
        let pos = path.segment(i);
        let current = maze.segment(pos.x, pos.y);

        for (i, dir) in Relative::iterator().enumerate() {
            let segment = current.relative(maze, dir);

            if segment.is_none() {
                continue;
            }

            if current.walls[i] {
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
            path.append(next.pos());

            if next.distance == 0 {
                break;
            }
        }

        assert_eq!(15, path.size());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1));
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2));
        assert_eq!(Veci { x: 3, y: 0 }, path.segment(3));
        assert_eq!(Veci { x: 4, y: 0 }, path.segment(4));
        assert_eq!(Veci { x: 5, y: 0 }, path.segment(5));
        assert_eq!(Veci { x: 6, y: 0 }, path.segment(6));
        assert_eq!(Veci { x: 7, y: 0 }, path.segment(7));
        assert_eq!(Veci { x: 7, y: 1 }, path.segment(8));
        assert_eq!(Veci { x: 7, y: 2 }, path.segment(9));
        assert_eq!(Veci { x: 7, y: 3 }, path.segment(10));
        assert_eq!(Veci { x: 7, y: 4 }, path.segment(11));
        assert_eq!(Veci { x: 7, y: 5 }, path.segment(12));
        assert_eq!(Veci { x: 7, y: 6 }, path.segment(13));
        assert_eq!(Veci { x: 7, y: 7 }, path.segment(14));
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(15));
    }

    #[test]
    fn next_suboptimal_greedy() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        loop {
            let next = pathfinder::next(&maze, &path);
            path.append(next.pos());

            if next.distance == 0 {
                break;
            }
        }

        maze.update_walls(0, 0, [true, false, true, true]);
        maze.update_walls(1, 0, [true, false, false, false]);
        maze.update_walls(2, 0, [true, true, true, false]);
        maze.update_walls(1, 1, [false, false, false, false]);

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1));
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(3));
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(4));
    }
}
