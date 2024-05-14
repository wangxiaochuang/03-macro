use anyhow::Result;

#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr; $n:expr) => {std::vec::from_elem($elem, $n)};
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}

fn main() -> Result<()> {
    // let v = my_vec![1; 5];
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];
    let vv = vec![1; 5];
    println!("{:?}", v);
    println!("{:?}", vv);

    Ok(())
}
