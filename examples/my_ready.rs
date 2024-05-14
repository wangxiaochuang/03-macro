use std::{future::Future, task::Poll};

#[macro_export]
macro_rules! my_ready {
    ($e:expr) => {
        match $e {
            Poll::Ready(val) => val,
            Poll::Pending => Poll::Pending,
        }
    };
}

struct MyFut {
    polled: bool,
    v: usize,
}

impl MyFut {
    fn new(v: usize) -> Self {
        Self { polled: false, v }
    }
}

impl Future for MyFut {
    type Output = usize;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.polled {
            Poll::Ready(self.v)
        } else {
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let fut = MyFut::new(42);
    println!("Final result: {:?}", fut.await);
}
