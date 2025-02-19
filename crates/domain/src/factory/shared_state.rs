use async_std::future::Future;

pub trait SharedStateFactory {
    fn new() -> impl Future<Output = Self> + Send
    where
        Self: std::marker::Sized;
}
