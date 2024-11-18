use crate::error::{ErrorType, RequestError};
use crate::value::Value;
use crate::{Error, RequestResult};
use serde::{Deserialize, Serialize};
use zbus::message::Type;
use zbus::names::{
    BusName, InterfaceName, MemberName, OwnedBusName, OwnedInterfaceName, OwnedMemberName,
};
use zvariant::{ObjectPath, OwnedObjectPath};

pub type RequestId = u64;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct MemberKey<'a> {
    #[serde(borrow)]
    pub destination: BusName<'a>,
    pub path: ObjectPath<'a>,
    pub interface: InterfaceName<'a>,
    pub name: MemberName<'a>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct OwnedMemberKey {
    pub destination: OwnedBusName,
    pub path: OwnedObjectPath,
    pub interface: OwnedInterfaceName,
    pub name: OwnedMemberName,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Clone)]
pub struct OwnedSignalKey {
    #[serde(flatten)]
    pub member: OwnedMemberKey,
    #[serde(default)]
    pub args: Vec<(u8, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum InputMessage<'a> {
    CallMethod {
        #[serde(default)]
        request_id: Option<RequestId>,
        #[serde(borrow)]
        destination: Option<BusName<'a>>,
        path: ObjectPath<'a>,
        interface: Option<InterfaceName<'a>>,
        method_name: MemberName<'a>,
        #[serde(default)]
        args: Vec<Value>,
    },
    SubscribeSignal {
        #[serde(default)]
        request_id: Option<RequestId>,
        #[serde(flatten)]
        key: OwnedSignalKey,
    },
    UnsubscribeSignal {
        #[serde(default)]
        request_id: Option<RequestId>,
        #[serde(flatten)]
        key: OwnedSignalKey,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all_fields = "camelCase")]
pub enum OutputMessage {
    MethodReturn {
        request_id: Option<RequestId>,
        args: Vec<Value>,
    },
    MethodError {
        request_id: Option<RequestId>,
        args: Vec<Value>,
    },
    Signal {
        #[serde(flatten)]
        key: OwnedSignalKey,
        args: Vec<Value>,
    },
    Success {
        request_id: Option<RequestId>,
    },
    Error {
        request_id: Option<RequestId>,
        error_type: ErrorType,
        message: String,
    },
}

impl<'a> InputMessage<'a> {
    pub fn request_id(&self) -> Option<RequestId> {
        match self {
            InputMessage::CallMethod { request_id, .. }
            | InputMessage::SubscribeSignal { request_id, .. }
            | InputMessage::UnsubscribeSignal { request_id, .. } => *request_id,
        }
    }
}

impl OutputMessage {
    pub fn from_method_call_result(
        msg: zbus::Message,
        request_id: Option<RequestId>,
    ) -> RequestResult<Self> {
        let args = Value::try_to_array_from_body(&msg.body())
            .map_err(|err| RequestError::new(request_id, err))?;
        match msg.message_type() {
            Type::MethodCall => Err(RequestError::new(
                request_id,
                Error::UnsupportedFormat(
                    "Method call message cannot be converted to Output message".into(),
                ),
            )),
            Type::MethodReturn => Ok(OutputMessage::MethodReturn { request_id, args }),
            Type::Error => Ok(OutputMessage::MethodError { request_id, args }),
            Type::Signal => Err(RequestError::new(
                request_id,
                Error::UnsupportedFormat("Signal cannot be converted to Output message".into()),
            )),
        }
    }
}
