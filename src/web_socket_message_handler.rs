use crate::error::{Error, RequestError};
use crate::message::{InputMessage, OutputMessage, OwnedSignalKey, RequestId};
use crate::state::WebSocketState;
use crate::value::Value;
use crate::{value, WebSocketEventHandler};
use crate::{RequestResult, Result};
use axum::extract::ws::Message;
use std::ops::ControlFlow;
use tracing::{error, info, instrument, trace, warn};
use zbus::names::{BusName, InterfaceName, MemberName};
use zbus::Proxy;
use zvariant::ObjectPath;

#[derive(Debug)]
pub struct WebSocketMessageHandler<'a> {
    dbus_connection: &'a zbus::Connection,
    state: &'a WebSocketState,
}

impl<'a> WebSocketMessageHandler<'a> {
    pub fn new(dbus_connection: &'a zbus::Connection, state: &'a WebSocketState) -> Self {
        Self {
            dbus_connection,
            state,
        }
    }

    #[instrument]
    async fn handle_message(&self, msg: Message) -> RequestResult<Option<OutputMessage>> {
        trace!("WebSocket message received: {:?}", msg);
        match msg {
            Message::Text(json) => {
                let input_message: InputMessage =
                    serde_json::from_str(&json).map_err(|err| RequestError::new(None, err))?;
                trace!("Input message received: {:?}", input_message);
                let request_id = input_message.request_id();
                return self
                    .handle_input_message(input_message)
                    .await
                    .map_err(|err| RequestError::new(request_id, err));
            }
            Message::Binary(_) => {
                return Err(
                    Error::UnsupportedFormat("Binary messages are unsupported".into()).into(),
                )
            }
            Message::Ping(_) => trace!("Ping received"),
            Message::Pong(_) => trace!("Pong received"),
            Message::Close(Some(cf)) => {
                if cf.code == axum::extract::ws::close_code::NORMAL {
                    info!("WebSocket connection closed: {}", cf.reason);
                } else {
                    warn!("WebSocket connection closed: {:?}", cf);
                }
            }
            Message::Close(None) => info!("WebSocket connection closed"),
        }
        Ok(None)
    }

    #[instrument]
    async fn handle_input_message(
        &self,
        input_message: InputMessage<'a>,
    ) -> Result<Option<OutputMessage>> {
        match input_message {
            InputMessage::CallMethod {
                request_id,
                destination,
                path,
                interface,
                method_name,
                args: body,
            } => {
                self.call_method(request_id, destination, path, interface, method_name, body)
                    .await
            }
            InputMessage::SubscribeSignal { request_id, key } => {
                self.subscribe_signal(request_id, key).await
            }
            InputMessage::UnsubscribeSignal { request_id, key } => {
                self.state.signals().remove(&key).await;
                Ok(Some(OutputMessage::Success { request_id }))
            }
        }
    }

    #[instrument]
    async fn call_method(
        &self,
        request_id: Option<RequestId>,
        destination: Option<BusName<'a>>,
        path: ObjectPath<'a>,
        interface: Option<InterfaceName<'a>>,
        method_name: MemberName<'a>,
        body: Vec<Value>,
    ) -> Result<Option<OutputMessage>> {
        let response = if body.is_empty() {
            self.dbus_connection
                .call_method(destination, path, interface, method_name, &())
                .await
        } else {
            let body = value::try_structure_from_fields(body)?;
            trace!("Message body: ({}){:?}", body.signature(), body);
            self.dbus_connection
                .call_method(destination, path, interface, method_name, &body)
                .await
        }
        .map_err(|err| RequestError::new(request_id, err))?;
        Ok(Some(OutputMessage::from_method_call_result(
            response, request_id,
        )?))
    }

    #[instrument]
    async fn subscribe_signal(
        &self,
        request_id: Option<RequestId>,
        OwnedSignalKey { member, args }: OwnedSignalKey,
    ) -> Result<Option<OutputMessage>> {
        let proxy = Proxy::new(
            self.dbus_connection,
            member.destination.clone(),
            member.path.clone(),
            member.interface.clone(),
        )
        .await?;
        let stream = if args.is_empty() {
            proxy.receive_signal(member.name.clone()).await
        } else {
            let args: Vec<(u8, &str)> = args.iter().map(|(i, s)| (*i, s.as_str())).collect();
            proxy
                .receive_signal_with_args(member.name.clone(), &args)
                .await
        }
        .map_err(|err| RequestError::new(request_id, err))?;
        self.state
            .signals()
            .insert(OwnedSignalKey { member, args }, stream)
            .await;
        Ok(Some(OutputMessage::Success { request_id }))
    }
}

impl<'a> WebSocketEventHandler<'a, Result<Option<Message>>> for WebSocketMessageHandler<'a> {
    #[instrument]
    async fn handle(
        &self,
        event: Result<Option<Message>>,
    ) -> ControlFlow<Option<OutputMessage>, Option<OutputMessage>> {
        match event {
            Ok(Some(msg)) => match self.handle_message(msg).await {
                Ok(Some(output_message)) => ControlFlow::Continue(Some(output_message)),
                Ok(None) => ControlFlow::Break(None),
                Err(err) => {
                    error!("Message handle error: {}", err);
                    ControlFlow::Continue(Some(err.into()))
                }
            },
            Ok(None) => ControlFlow::Continue(None),
            Err(err) => ControlFlow::Break(Some(err.into())),
        }
    }
}
