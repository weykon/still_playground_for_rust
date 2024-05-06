use crate::Module;

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

pub fn run () {
    
}