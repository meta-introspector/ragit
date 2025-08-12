fn main() {
    // Some dummy code
    let result = my_async_operation();
    ready!(result);
}

fn my_async_operation() -> Poll<u32> {
    // ...
    Poll::Ready(42)
}

enum Poll<T> {
    Ready(T),
    Pending,
}
