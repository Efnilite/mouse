use heapless::Vec;
use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder;
use mouse::vec::{Vecf, Vecu};

fn main() {
    let mut pos = Vecf::new();
    let mut heading = Vecf::new();

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
            let next: &Vec<Vecu, 256> = result.unwrap_stuck();
            println!("{:?}", next);
            path.append_all(&next);
            pathfinder::update_distances(&mut maze, &path);
        }
    }

    path.optimize();
    path.time_to_complete();

    println!("{:?}", maze);
}
