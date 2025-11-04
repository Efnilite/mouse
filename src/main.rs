use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder;
use mouse::pathfinder::Target;
use mouse::vec::{Vecf, Vecu};
use std::ptr::null;

fn main() {
    let _pos = Vecf::new();
    let _heading = Vecf::new();

    let mut maze = Maze::new();
    let mut first = Path::new();

    first.append(Vecu::new());
    maze.update_walls(0, 0, [true, false, true, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, true, true, false]);
    maze.update_walls(1, 1, [false, false, false, false]);

    // first deep dive
    loop {
        let result = pathfinder::next(&maze, &first);

        println!("{:?}", first);
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
    }

    // todo find first unvisited node
    first.optimize();
    println!("xxx {:?}", first);
    //
    // let mut maze = Maze::with_walls(Target::Origin, maze);
    // let mut second = Path::new();
    // second.append(first.head().unwrap());
    // let vec = pathfinder::nearest_unvisited(&maze, &first);
    //
    // println!("{:?}", maze);
    // println!("{:?}", second);
    //
    // // second
    // loop {
    //     let result = pathfinder::next(&maze, &second);
    //     println!("{:?}", second);
    //
    //     match result {
    //         pathfinder::Result::Found(next) => {
    //             second.append(next.pos());
    //
    //             if next.pos() == Vecu::new() {
    //                 break;
    //             }
    //         }
    //         pathfinder::Result::Stuck(next) => {
    //             second.append_all(&next);
    //             pathfinder::update_distances(&mut maze, &second);
    //         }
    //     }
    // }
    //
    // second.optimize();

    // println!("{:?}", second);
    println!("{:?}", maze);
}
