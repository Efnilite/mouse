use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder::next;
use mouse::vec::{Vecf, Vecu};
use mouse::MAZE_SIZE;
use heapless::{Vec};

fn main() {
    let mut pos = Vecf::new();
    let mut heading = Vecf::new();

    let mut maze = Maze::new();
    let mut path = Path::new();

    path.append(Vecu::new());
    maze.update_walls(0, 0, [true, false, true, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, true, true, false]);
    maze.update_walls(1, 1, [false, false, false, false]);

    loop {
        let result = next(&maze, &path);

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
        }
    }

    path.optimize();
    path.time_to_complete();

    println!("{:?}", path);
}