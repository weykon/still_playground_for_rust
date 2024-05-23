struct SnakeGame {
    snakes: Vec<Snake>,
    game_over: bool,
}
struct Snake {
    data: SnakeData,
    body: SnakeBody,
}

struct SnakeBody {
    head_move_points: Vec<MovePoint>,
    head: Point,
    tail: Vec<Point>,
}

struct MovePoint {
    point: Point,
    direction: Vector,
}
pub struct Vector {
    pub x: i32,
    pub y: i32,
}
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct SnakeData {
    score: i32,
}
pub fn run () { 
    
}