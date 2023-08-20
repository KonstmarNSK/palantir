use std::future::Future;
use std::io::Error;
use tokio::runtime::Runtime as TRuntime;

pub struct TokioRuntime(TRuntime);

impl iced::executor::Executor for TokioRuntime {
    fn new() -> Result<Self, Error> {
        let runtime = TRuntime::new()?;
        Ok(TokioRuntime(runtime))
    }

    #[allow(clippy::let_underscore_future)]
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let _ = TRuntime::spawn(&self.0, future);
    }

    fn enter<R>(&self, f: impl FnOnce() -> R) -> R {
        let _guard = tokio::runtime::Runtime::enter(&self.0);
        f()
    }
}