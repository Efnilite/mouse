use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder;
use mouse::vec::{Vecf, Vecu};

fn main() {
    let _pos = Vecf::new();
    let _heading = Vecf::new();

    let mut maze = Maze::new();
    let mut path = Path::new();

    path.append(Vecu::new());
    maze.update_walls(0, 0, [true, false, false, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, false, true, false]);
    maze.update_walls(3, 0, [true, true, false, false]);

    maze.update_walls(0, 1, [false, true, false, true]);
    maze.update_walls(1, 1, [false, false, true, true]);
    maze.update_walls(2, 1, [true, false, true, false]);
    maze.update_walls(3, 1, [false, true, true, false]);

    // first deep dive
    loop {
        let result = pathfinder::next(
            &maze,
            &path,
            |a, b| a.distance < b.distance,
            |a, b| a.distance <= b.distance,
        );

        match result {
            pathfinder::Result::Found(next) => {
                path.append(next.pos());

                if next.distance == 0 {
                    break;
                }
                continue;
            }
            pathfinder::Result::Stuck(next) => {
                path.append_all(&next);
                pathfinder::update_distances(&mut maze, &path);
            }
        }
    }

    // todo find first unvisited node

    // second
    loop {
        let result = pathfinder::next(
            &maze,
            &path,
            |a, b| a.distance > b.distance,
            |a, b| a.distance >= b.distance,
        );

        match result {
            pathfinder::Result::Found(next) => {
                path.append(next.pos());

                if next.pos() == Vecu::new() {
                    break;
                }
                continue;
            }
            pathfinder::Result::Stuck(next) => {
                path.append_all(&next);
                pathfinder::update_distances(&mut maze, &path);
            }
        }
    }

    path.optimize();
    path.time_to_complete();

    println!("{:?}", maze);
}
