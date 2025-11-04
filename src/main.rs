use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder;
use mouse::pathfinder::Target;
use mouse::vec::{Vecf, Vecu};

fn main() {
    let _pos = Vecf::new();
    let _heading = Vecf::new();

    let mut maze = Maze::new();
    let mut first = Path::new();

    first.append(Vecu::new());
    maze.update_walls(0, 0, [true, false, false, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, false, true, false]);
    maze.update_walls(3, 0, [true, true, false, false]);

    maze.update_walls(0, 1, [false, true, false, true]);
    maze.update_walls(1, 1, [false, false, true, true]);
    maze.update_walls(2, 1, [true, false, true, false]);
    maze.update_walls(3, 1, [false, true, true, false]);
    maze.update_walls(7, 7, [true, false, false, true]);
    maze.update_walls(8, 7, [true, true, false, false]);

    // first deep dive
    loop {
        let result = pathfinder::next(&maze, &first, Target::Center);

        match result {
            pathfinder::Result::Found(next) => {
                first.append(next.pos());

                if next.distance == 0 {
                    break;
                }
            }
            pathfinder::Result::Stuck(next) => {
                first.append_all(&next);
                pathfinder::update_distances(&mut maze, &first);
            }
        }
        println!("{:?}", first);
    }

    // todo find first unvisited node
    first.optimize();
    println!("{:?}", first);
    let mut second = pathfinder::nearest_unvisited(&maze, &first);

    println!("{:?}", second);

    // second
    loop {
        let result = pathfinder::next(&maze, &second, Target::Origin);

        if second.len() % 10 == 0 {
            println!("{:?}", second);
        }

        match result {
            pathfinder::Result::Found(next) => {
                second.append(next.pos());

                if next.pos() == Vecu::new() {
                    break;
                }
            }
            pathfinder::Result::Stuck(next) => {
                second.append_all(&next);
                pathfinder::update_distances(&mut maze, &second);
            }
        }
    }

    second.optimize();

    // println!("{:?}", second);
    println!("{:?}", maze);
}
