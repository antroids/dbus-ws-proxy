use crate::message::{OutputMessage, OwnedSignalKey};
use crate::value::Value;
use crate::WebSocketEventHandler;
use std::ops::ControlFlow;
use tracing::instrument;
use zbus::Message;

#[derive(Default, Debug)]
pub struct SignalHandler {}

impl WebSocketEventHandler<'static, (OwnedSignalKey, Message)> for SignalHandler {
    #[instrument]
    async fn handle(
        &self,
        (key, message): (OwnedSignalKey, Message),
    ) -> ControlFlow<Option<OutputMessage>, Option<OutputMessage>> {
        match Value::try_to_array_from_body(&message.body()) {
            Ok(args) => ControlFlow::Continue(Some(OutputMessage::Signal { key, args })),
            Err(err) => ControlFlow::Continue(Some(err.into())),
        }
    }
}
