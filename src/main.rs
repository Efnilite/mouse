use mouse::maze::Maze;
use mouse::path::Path;
use mouse::pathfinder;
use mouse::pathfinder::Target;
use mouse::vec::Vecu;

const DT: f64 = 0.01;
const MU: f64 = 0.1;
const M: f64 = 0.1;
const W: f64 = 7.;

fn main() {
    let mut maze = Maze::new();
    let mut first = Path::new();

    first.append(Vecu::new());

    let mut t = 0.;

    let mut px;
    let mut py;
    let mut ax;
    let mut ay;
    let mut vx;
    let mut vy;

    loop {
        let mut fresx = 0.;
        let mut fresy = 0.;

        ax = fresx / M;
        ay = fresy / M;

        vx = ax * DT;
        vy = ay * DT;

        px = vx * DT;
        py = vy * DT;

        t = t * DT;

        println!("{t} -> {px} {py}");
    }
}

fn _find() {
    let mut maze = Maze::new();
    let mut first = Path::new();

    maze.update_walls(0, 0, [true, false, true, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, true, true, false]);
    maze.update_walls(1, 1, [false, false, false, false]);

    // first deep dive
    loop {
        let result = pathfinder::next(&maze, &first);

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

    first.optimize();

    let mut maze = Maze::with_walls(Target::Origin, maze);
    let mut second = Path::new();
    second.append(first.head().unwrap());
    second.append_all(&pathfinder::nearest_unvisited(&maze, &first));

    // second
    loop {
        let result = pathfinder::next(&maze, &second);

        match result {
            pathfinder::Result::Found(next) => {
                second.append(next.pos());

                if next.distance == 0 {
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

    println!("{:?}", first);
    println!("{:?}", second);
    println!("{:?}", maze);
}
