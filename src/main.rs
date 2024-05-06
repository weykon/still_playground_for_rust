mod print_it;
mod game_attack;
mod payment;
mod unkown_type;
trait Dimension {}

struct Dimension2 {
    x: i32,
    y: i32,
}

struct Dimension3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Dimension for Dimension2 {}
impl Dimension for Dimension3 {}

struct DataPoint<D: Dimension, T> {
    dimension: D,
    data: T,
}
struct Map<D: Dimension, T> {
    width: i32,
    height: i32,
    data: Vec<DataPoint<D, T>>,
}

fn main() {
    println!("Hello, world!");

   let map_2d = Map {
        width: 10,
        height: 10,
        data: vec![DataPoint {
            dimension: Dimension2 { x: 1, y: 2 },
            data: 1,
        }],
    };

    let map_3d = Map {
        width: 10,
        height: 10,
        data: vec![DataPoint {
            dimension: Dimension3 { x: 1, y: 2, z: 3 },
            data: 1,
        }],
    };
}
