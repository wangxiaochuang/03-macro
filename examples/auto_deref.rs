use rustpl::AutoDeref;

#[allow(dead_code)]
#[derive(Debug, AutoDeref)]
#[auto_deref(mutable = true, field = "inner")]
pub struct RespBulkString {
    inner: String,
    nothing: (),
}
fn main() {
    let s = RespBulkString {
        inner: "hello".to_string(),
        nothing: (),
    };

    println!("{:?}", s);
}
