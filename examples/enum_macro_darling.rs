use rustpl::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(i32),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: i32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 10.into();
    println!("{:?}\n{:?}", up, left);
}

impl DirectionUp {
    fn new(speed: i32) -> Self {
        Self { speed }
    }
}
