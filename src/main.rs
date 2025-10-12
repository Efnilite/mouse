use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder::{next, next_unvisited};
use mouse::vec::{Vecf, Vecu};

fn main() {
    let mut pos = Vecf::new();
    let mut heading = Vecf::new();

    let maze = Maze::new();
    let mut path = Path::new();

    path.append(Vecu::new());

    loop {
        let result = next(&maze, &path);

        if result.is_found() {
            let next = result.unwrap();
            path.append(next.pos());

            if next.distance == 0 {
                break;
            }
            continue;
        }

        let mut segments = next_unvisited(&maze, &path);
        segments.remove(0);
        path.append_all(segments);
    }

    path.optimize();
    path.time_to_complete();

    println!("{:?}", path);
}