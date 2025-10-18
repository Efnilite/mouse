use crate::map::Map;
use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;
use crate::vec::Vecu;
use crate::MAZE_SIZE;
use heapless::{Deque, Vec};

/// The result of an attempted pathfinding using [next].
pub enum Result {
    /// Indicates that a segment has been found that is not a neighbour of the head of the path.
    /// Contains the new segment and the path from the head to the segment.
    Stuck(Vec<Vecu, MAZE_SIZE>),

    /// Indicates that a valid neighbour has been found as the next segment.
    Found(Segment),
}

impl Result {
    /// Whether this result is a dead end or not.
    pub fn is_dead_end(&self) -> bool {
        matches!(*self, Result::Stuck(_))
    }

    /// Whether this result contains a found [Segment] or not.
    pub fn is_found(&self) -> bool {
        matches!(*self, Result::Found(_))
    }

    /// Unwraps the found value.
    pub fn unwrap_found(self) -> Segment {
        match self {
            Result::Found(s) => s,
            _ => panic!("Called `Result::unwrap_found` on a non-Found value"),
        }
    }

    /// Unwraps the stuck value.
    pub fn unwrap_stuck(&self) -> &Vec<Vecu, MAZE_SIZE> {
        match self {
            Result::Stuck(s) => s,
            _ => panic!("Called `Result::unwrap_stuck` on a non-Stuck value"),
        }
    }
}

/// Attempts to find the next segment based on `maze` and the taken `path`.
///
/// ### Description
///
/// Returns the segment `n` with the smallest non-strict distance to the target
/// that is reachable from and a neighbour of `p`. `p` is any value in `path`.
///
/// ### Implementation
///
/// For every element in path, starting from the beginning, all neighbours are checked.
/// If a neighbour `n` is reachable and has not yet been visited, it is considered as a potential
/// minimal segment. If `n` has a lower distance than the current minimal segment, `n` becomes the
/// minimal segment.
///
/// If the minimal segment is disconnected from the head, returns a path to the minimal segment.
/// This is not guaranteed to be the shortest path.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
///
/// ### Returns
///
/// - [Result::Found] - A valid next segment has been found.
/// - [Result::Stuck] - A valid next segment has been found, but it is not directly attached
/// to the head of `path`. Returns the path to the valid next segment.
pub fn next(maze: &Maze, path: &Path) -> Result {
    // the smallest segment so far
    let mut min_segment = Segment::new();
    // the distance from min segment to the current segment in the loop
    let mut min_segment_distance = 0usize;

    for i in (0..path.len()).rev() {
        let current = maze.segment_vec(path.segment(i).expect("Failed to find path segment"));

        'dirs: for (j, dir) in Relative::iter().enumerate() {
            if current.walls[j] {
                continue 'dirs;
            }

            let segment = current.relative(maze, dir);

            if segment.is_none() {
                continue 'dirs;
            }

            let segment = segment.unwrap();

            if path.contains(segment.pos()) {
                continue 'dirs;
            }

            if segment.distance < min_segment.distance {
                min_segment = segment;
                min_segment_distance = i;
            }
        }
    }

    if maze.segment_vec(path.head().unwrap()).distance <= min_segment.distance {
        let mut to_min: Vec<Vecu, MAZE_SIZE> = Vec::new();
        for i in (min_segment_distance..path.len() - 1).rev() { // - 1 to skip head
            to_min.push(path.segment(i).unwrap()).unwrap();
        }
        to_min.push(min_segment.pos()).unwrap();

        Result::Stuck(to_min)
    } else {
        Result::Found(min_segment)
    }
}

/// Updates the distances in `maze` when a dead end is reached.
///
/// ### Description
///
/// Updates the distances in a dead end by BFS such that, for any node `n` in the dead end,
/// `d(n, goal) == d(start, path.segment(path.len() - 2)) + d(path.segment(path.len() - 2), n)`.
/// Here, `d` is guaranteed to be the shortest distance between two nodes.
///
/// ### Implementation
///
/// Every element in the path that is between the last two occurrences of the `root` node
/// `path.segment(path.len() - 2)` is added to the possible options. If no two
/// occurrences are found, returns and does nothing.
///
/// Then, the last occurrence of `root` is added to `to_explore`, which details which
/// nodes are to be explored, and `root` is added to `explored`, which details which nodes
/// to not explore again.
///
/// In a loop, BFS is performed. For every node in the options, its distance is updated to
/// the parent node's distance, plus 1. Nodes that have been explored are added to `explored`.
/// When there are no more nodes to explore, exits.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
pub fn update_distances(maze: &mut Maze, path: &Path) {
    assert!(!path.optimized(), "Only unoptimized paths can have distances updated");

    let head_idx = path.len() - 2;
    let root = path.segment(head_idx).unwrap();
    let mut previous_head_idx = usize::MAX; // the first time head re-appeared
    for i in (0..path.len() - 2).rev() {
        if path.segment(i).unwrap() == root {
            previous_head_idx = i;
            break;
        }
    }

    // updating distances cannot be applied if there is no loop
    if previous_head_idx == head_idx || previous_head_idx == usize::MAX {
       return;
    }

    // contains all vecs that can be explored.
    let mut options: Map<bool> = Map::new();
    for i in previous_head_idx..head_idx {
        options.insert(path.segment(i).unwrap(), true);
    }

    let mut to_explore: Deque<Vecu, MAZE_SIZE> = Deque::new();
    // contains the vecs that have been explored, with the value being the parent vec.
    // for the root, value is `None`.
    let mut explored: Map<Option<Vecu>> = Map::new();

    {
        explored.insert(root, None);
        to_explore.push_back(root).unwrap();
    }

    while !to_explore.is_empty() {
        let current_pos = to_explore
            .pop_front()
            .expect("Failed to find unvisited node in entire maze");
        let current_segment = maze.segment_vec(current_pos);

        // check all directions for unvisited segments
        'dirs: for (i, dir) in Relative::iter().enumerate() {
            if current_segment.walls[i] {
                continue 'dirs;
            }

            let relative = current_segment.relative(maze, dir);
            if relative.is_none() {
                // wall or OOB
                continue 'dirs;
            }

            let segment = relative.unwrap();
            let new_pos = segment.pos();
            if explored.contains_key(&new_pos) || !options.contains_key(&new_pos) {
                // already explored or not in the options
                continue 'dirs;
            }

            explored.insert(
                new_pos,
                Some(current_pos)
            );
            to_explore.push_back(new_pos).unwrap();

            maze.update_distance(new_pos.x, new_pos.y, current_segment.distance + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use crate::path::Path;
    use crate::pathfinder;
    use crate::vec::Vecu;

    /// Finds any segment that has a distance of zero.
    /// Updates `path` on the way.
    ///
    /// ### Arguments
    ///
    /// - `maze` - The maze.
    /// - `path` - The path that has been taken so far. Is updated by this method.
    fn find(maze: &mut Maze, path: &mut Path) {
        path.append(Vecu::new());

        loop {
            let result = pathfinder::next(&maze, &path);

            if result.is_found() {
                let next = result.unwrap_found();
                println!("{:?}", next);
                path.append(next.pos());

                if next.distance == 0 {
                    break;
                }
                continue;
            } else {
                let next: &heapless::Vec<Vecu, 256> = result.unwrap_stuck();
                println!("{:?}", next);
                path.append_all(next);
                pathfinder::update_distances(maze, &path);
            }
        }
    }

    #[test]
    fn next() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        find(&mut maze, &mut path);

        assert_eq!(15, path.len());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 4, y: 0 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 5, y: 0 }, path.segment(5).unwrap());
        assert_eq!(Vecu { x: 6, y: 0 }, path.segment(6).unwrap());
        assert_eq!(Vecu { x: 7, y: 0 }, path.segment(7).unwrap());
        assert_eq!(Vecu { x: 7, y: 1 }, path.segment(8).unwrap());
        assert_eq!(Vecu { x: 7, y: 2 }, path.segment(9).unwrap());
        assert_eq!(Vecu { x: 7, y: 3 }, path.segment(10).unwrap());
        assert_eq!(Vecu { x: 7, y: 4 }, path.segment(11).unwrap());
        assert_eq!(Vecu { x: 7, y: 5 }, path.segment(12).unwrap());
        assert_eq!(Vecu { x: 7, y: 6 }, path.segment(13).unwrap());
        assert_eq!(Vecu { x: 7, y: 7 }, path.segment(14).unwrap());
        assert!(path.segment(15).is_none());
    }

    #[test]
    fn next_guided() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // ###
        // # #
        // # #
        maze.update_walls(0, 0, [true, true, false, true]);
        maze.update_walls(0, 1, [false, true, false, true]);

        find(&mut maze, &mut path);

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 0, y: 1 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 0, y: 2 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 1, y: 2 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 2, y: 2 }, path.segment(4).unwrap());
    }

    #[test]
    fn next_guided_diagonal() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // ######
        // #  ###
        // ##  ##
        // ###  #
        maze.update_walls(0, 0, [true, false, true, true]);
        maze.update_walls(1, 0, [true, true, false, false]);
        maze.update_walls(1, 1, [false, false, true, true]);
        maze.update_walls(2, 1, [true, true, false, false]);
        maze.update_walls(2, 2, [false, false, true, true]);
        maze.update_walls(3, 2, [true, true, false, false]);

        find(&mut maze, &mut path);

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 2, y: 1 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 2, y: 2 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 3, y: 2 }, path.segment(5).unwrap());
    }

    #[test]
    fn general_deadend() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // #####
        // #   #
        // ## ##
        maze.update_walls(0, 0, [true, false, true, true]);
        maze.update_walls(1, 0, [true, false, false, false]);
        maze.update_walls(2, 0, [true, true, true, false]);
        maze.update_walls(1, 1, [false, false, false, false]);

        find(&mut maze, &mut path);

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(4).unwrap());

        println!("{:?}", maze);

        assert_eq!(14, maze.segment(0, 0).distance);
        assert_eq!(13, maze.segment(1, 0).distance);
        assert_eq!(14, maze.segment(2, 0).distance);
        assert_eq!(12, maze.segment(1, 1).distance);
    }

    #[test]
    fn general_nontrivial_deadend() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // #######
        // #     #
        // # # # #
        // # #   #
        // # #####
        maze.update_walls(0, 0, [true, false, false, true]);
        maze.update_walls(1, 0, [true, false, false, false]);
        maze.update_walls(2, 0, [true, false, true, false]);
        maze.update_walls(3, 0, [true, true, false, false]);

        maze.update_walls(0, 1, [false, true, false, true]);
        maze.update_walls(1, 1, [false, false, true, true]);
        maze.update_walls(2, 1, [true, false, true, false]);
        maze.update_walls(3, 1, [false, true, true, false]);

        find(&mut maze, &mut path);

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 3, y: 1 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 2, y: 1 }, path.segment(5).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(6).unwrap());
        assert_eq!(Vecu { x: 2, y: 1 }, path.segment(7).unwrap());
        assert_eq!(Vecu { x: 3, y: 1 }, path.segment(8).unwrap());
        assert_eq!(Vecu { x: 3, y: 0 }, path.segment(9).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(10).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(11).unwrap());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(12).unwrap());
        assert_eq!(Vecu { x: 0, y: 1 }, path.segment(13).unwrap());
    }
}
