pub mod bindings {
    wasmtime::component::bindgen!({
        path: "wit",
        interfaces: "
            import wasi:tracing/producer;
        ",
        tracing: false,
        async: false,
        with: {
            // "wasi:io/error": wasmtime_wasi::preview2::bindings::io::error,
            // "wasi:io/streams": wasmtime_wasi::preview2::bindings::io::streams,
            // "wasi:io/poll": wasmtime_wasi::preview2::bindings::io::poll,

            // "wasi:http/types/outgoing-body": super::body::HostOutgoingBody,
            // "wasi:http/types/future-incoming-response": super::types::HostFutureIncomingResponse,
            // "wasi:http/types/outgoing-response": super::types::HostOutgoingResponse,
            // "wasi:http/types/future-trailers": super::body::HostFutureTrailers,
            // "wasi:http/types/incoming-body": super::body::HostIncomingBody,
            // "wasi:http/types/incoming-response": super::types::HostIncomingResponse,
            // "wasi:http/types/response-outparam": super::types::HostResponseOutparam,
            // "wasi:http/types/outgoing-request": super::types::HostOutgoingRequest,
            // "wasi:http/types/incoming-request": super::types::HostIncomingRequest,
            // "wasi:http/types/fields": super::types::HostFields,
            // "wasi:http/types/request-options": super::types::HostRequestOptions,
        }
    });

    pub use wasi::tracing;
}

pub fn add_tracing_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> anyhow::Result<()>
where
    T: WasiTracingView + bindings::wasi::tracing::producer::Host,
{
    bindings::tracing::producer::add_to_linker(l, |t| t)?;

    Ok(())
}

impl<T: WasiTracingView> crate::bindings::tracing::producer::Host for T {
    fn send_event(&mut self, event: Vec<u8>) -> wasmtime::Result<()> {
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

pub trait WasiTracingView: Send {
    fn ctx(&mut self) -> &mut WasiTracingCtx;
}
