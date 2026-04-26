use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use web_time::Instant;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use std::time::Instant;

pub type Sleep = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[derive(Debug, Clone, Copy)]
pub struct Elapsed;

impl std::fmt::Display for Elapsed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "elapsed")
    }
}

impl std::error::Error for Elapsed {}

pub fn sleep(duration: Duration) -> Sleep {
    sleep_impl(duration)
}

pub fn sleep_until(deadline: Instant) -> Sleep {
    sleep(deadline.saturating_duration_since(Instant::now()))
}

pub async fn timeout<F>(duration: Duration, future: F) -> Result<F::Output, Elapsed>
where
    F: Future,
{
    tokio::select! {
        output = future => Ok(output),
        () = sleep(duration) => Err(Elapsed),
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn sleep_impl(duration: Duration) -> Sleep {
    Box::pin(tokio::time::sleep(duration))
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn sleep_impl(duration: Duration) -> Sleep {
    use js_sys::{Function, Reflect, global};
    use tokio::sync::oneshot;
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};

    if duration.is_zero() {
        return Box::pin(async {});
    }

    let (sender, receiver) = oneshot::channel();
    let callback = Closure::once_into_js(move || {
        let _ = sender.send(());
    });

    let timeout_ms = duration.as_millis().min(i32::MAX as u128) as i32;
    let global = global();
    let set_timeout = Reflect::get(&global, &JsValue::from_str("setTimeout"))
        .expect("global setTimeout should exist")
        .dyn_into::<Function>()
        .expect("setTimeout should be a function");
    let _ = set_timeout.call2(
        &global,
        callback.as_ref(),
        &JsValue::from_f64(timeout_ms as f64),
    );

    Box::pin(async move {
        let _ = receiver.await;
    })
}
