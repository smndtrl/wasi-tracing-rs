pub mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        interfaces: "
            import wasi:tracing/producer;
        ",
        // tracing: true,
        // async: false,
        trappable_imports: true
    });

    pub use wasi::tracing;
}

fn type_annotate_tracing<T, F>(val: F) -> F
where
    F: Fn(&mut T) -> &mut dyn WasiTracingView,
{
    val
}

pub fn add_tracing_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
where
    T: WasiTracingView //+  bindings::wasi::tracing::producer::Host,
{
    let closure = type_annotate_tracing::<T, _>(|t| t);
    crate::bindings::tracing::producer::add_to_linker_get_host(l, closure)?;

    Ok(())
}

impl crate::bindings::tracing::producer::Host for dyn WasiTracingView + '_  {
    fn send_event(&mut self, event: Vec<u8>) -> Result<(), wasmtime::Error> {
        let event = serde_json::from_slice(&event).map_err(wasmtime::Error::new)?;
        self.ctx()
            .0
            .try_receive(event)
            .map_err(wasmtime::Error::new)?;
        Ok(())
    }
}

pub use tracing_tunnel::TracingEventReceiver;

pub struct WasiTracingCtx(pub(crate) TracingEventReceiver);

impl WasiTracingCtx {
    pub fn new(subscriber: TracingEventReceiver) -> Self {
        Self(subscriber)
    }
}

pub trait WasiTracingView:  Send  {
    fn ctx(&mut self) -> &mut WasiTracingCtx;
}

// impl<T: ?Sized + WasiTracingView> WasiTracingView for &mut T {
//     fn ctx(&mut self) -> &mut WasiTracingCtx {
//         T::ctx(self)
//     }

// }