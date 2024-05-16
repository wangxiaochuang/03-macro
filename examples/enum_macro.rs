use rustpl::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(T),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<i32> = 10.into();
    println!("{:?}\n{:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}
