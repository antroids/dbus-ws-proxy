use crate::value;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use thiserror::Error;
use zbus::message::Body;
use zvariant::signature::Child;
use zvariant::OwnedValue;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The elements have different types")]
    ElementsTypeIsDifferent,
    #[error("The structure is empty")]
    EmptyStructure,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    Primitive(PrimitiveValue),
    Container(ContainerValue),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveValue {
    U8(u8),
    Bool(bool),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F64(Decimal),
    String(String),
    Signature(zvariant::Signature),
    ObjectPath(zvariant::OwnedObjectPath),
    Fd(zvariant::OwnedFd),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ContainerValue {
    Variant { value: Box<Value> },
    Array(Array),
    Dict(Dict),
    Struct { value: Vec<Value> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Array {
    ValueType(#[serde(default)] value::ValueType),
    Value(Vec<Value>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Dict {
    ValueType {
        #[serde(default = "default_dict_key", rename = "keyType")]
        key_type: Box<PrimitiveType>,
        #[serde(default, rename = "valueType")]
        value_type: Box<ValueType>,
    },
    Value {
        value: HashMap<String, Value>, // Only string keys are supported for non-empty maps
    },
}

#[derive(Serialize, Deserialize, Debug, Default, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PrimitiveType {
    U8,
    Bool,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F64,
    #[default]
    String,
    Signature,
    ObjectPath,
    Fd,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ContainerType {
    #[default]
    Variant,
    Array {
        #[serde(default, rename = "valueType")]
        value_type: Box<ValueType>,
    },
    Dict {
        #[serde(default = "default_dict_key", rename = "keyType")]
        key_type: Box<PrimitiveType>,
        #[serde(default, rename = "valueType")]
        value_type: Box<ValueType>,
    },
    Struct {
        fields: Vec<ValueType>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ValueType {
    Primitive(PrimitiveType),
    Container(ContainerType),
}

fn default_dict_key() -> Box<PrimitiveType> {
    Box::new(PrimitiveType::default())
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::Container(ContainerType::default())
    }
}

impl Default for Array {
    fn default() -> Self {
        Self::ValueType(ValueType::default())
    }
}

impl From<PrimitiveValue> for Value {
    fn from(value: PrimitiveValue) -> Self {
        Value::Primitive(value)
    }
}

impl From<ContainerValue> for Value {
    fn from(value: ContainerValue) -> Self {
        Value::Container(value)
    }
}

impl From<PrimitiveType> for ValueType {
    fn from(value: PrimitiveType) -> Self {
        ValueType::Primitive(value)
    }
}

impl From<ContainerType> for ValueType {
    fn from(value: ContainerType) -> Self {
        ValueType::Container(value)
    }
}

impl From<Array> for ContainerValue {
    fn from(value: Array) -> Self {
        Self::Array(value)
    }
}

impl From<Dict> for ContainerValue {
    fn from(value: Dict) -> Self {
        Self::Dict(value)
    }
}

impl PrimitiveValue {
    fn type_(&self) -> PrimitiveType {
        match self {
            PrimitiveValue::U8(_) => PrimitiveType::U8,
            PrimitiveValue::Bool(_) => PrimitiveType::Bool,
            PrimitiveValue::I16(_) => PrimitiveType::I16,
            PrimitiveValue::U16(_) => PrimitiveType::U16,
            PrimitiveValue::I32(_) => PrimitiveType::I32,
            PrimitiveValue::U32(_) => PrimitiveType::U32,
            PrimitiveValue::I64(_) => PrimitiveType::I64,
            PrimitiveValue::U64(_) => PrimitiveType::U64,
            PrimitiveValue::F64(_) => PrimitiveType::F64,
            PrimitiveValue::String(_) => PrimitiveType::String,
            PrimitiveValue::Signature(_) => PrimitiveType::Signature,
            PrimitiveValue::ObjectPath(_) => PrimitiveType::ObjectPath,
            PrimitiveValue::Fd(_) => PrimitiveType::Fd,
        }
    }
}

impl ContainerValue {
    fn type_(&self) -> ContainerType {
        match self {
            ContainerValue::Variant { .. } => ContainerType::Variant,
            ContainerValue::Array(array) => ContainerType::Array {
                value_type: Box::new(match array {
                    Array::ValueType(value_type) => value_type.clone(),
                    Array::Value(value) => {
                        value.first().map(|first| first.type_()).unwrap_or_default()
                    }
                }),
            },
            ContainerValue::Dict(dict) => match dict {
                Dict::ValueType {
                    key_type,
                    value_type,
                } => ContainerType::Dict {
                    key_type: key_type.clone(),
                    value_type: value_type.clone(),
                },
                Dict::Value { value } => {
                    let value_type = Box::new(
                        value
                            .iter()
                            .next()
                            .map(|(_, v)| v.type_())
                            .unwrap_or_default(),
                    );
                    ContainerType::Dict {
                        key_type: Box::new(PrimitiveType::String),
                        value_type,
                    }
                }
            },
            ContainerValue::Struct { value } => ContainerType::Struct {
                fields: value.iter().map(|v| v.type_()).collect(),
            },
        }
    }
}

impl Value {
    fn type_(&self) -> ValueType {
        match self {
            Value::Primitive(p) => ValueType::Primitive(p.type_()),
            Value::Container(c) => ValueType::Container(c.type_()),
        }
    }
}

// ZBus integration
// value -> zbus
impl Value {
    pub fn try_from_body(body: &Body) -> crate::Result<Option<Self>> {
        Ok(if body.is_empty() {
            None
        } else {
            Some(zvariant::Value::Structure(body.deserialize()?).into())
        })
    }

    pub fn try_to_array_from_body(body: &Body) -> crate::Result<Vec<Self>> {
        if body.is_empty() {
            Ok(vec![])
        } else {
            let structure: zvariant::Structure = body.deserialize()?;
            Ok(structure
                .into_fields()
                .iter()
                .map(|field| field.into())
                .collect())
        }
    }
}

impl From<PrimitiveType> for zvariant::Signature {
    fn from(value: PrimitiveType) -> Self {
        match value {
            PrimitiveType::U8 => Self::U8,
            PrimitiveType::Bool => Self::Bool,
            PrimitiveType::I16 => Self::I16,
            PrimitiveType::U16 => Self::U16,
            PrimitiveType::I32 => Self::I32,
            PrimitiveType::U32 => Self::U32,
            PrimitiveType::I64 => Self::I64,
            PrimitiveType::U64 => Self::U64,
            PrimitiveType::F64 => Self::F64,
            PrimitiveType::String => Self::Str,
            PrimitiveType::Signature => Self::Signature,
            PrimitiveType::ObjectPath => Self::ObjectPath,
            PrimitiveType::Fd => Self::Fd,
        }
    }
}

impl From<ValueType> for Child {
    fn from(value: ValueType) -> Self {
        let signature: zvariant::Signature = value.into();
        signature.into()
    }
}

impl From<Box<ValueType>> for Child {
    fn from(value: Box<ValueType>) -> Self {
        (*value).into()
    }
}

impl From<PrimitiveType> for Child {
    fn from(value: PrimitiveType) -> Self {
        let signature: zvariant::Signature = value.into();
        signature.into()
    }
}

impl From<Box<PrimitiveType>> for Child {
    fn from(value: Box<PrimitiveType>) -> Self {
        (*value).into()
    }
}

impl From<ContainerType> for zvariant::Signature {
    fn from(value: ContainerType) -> Self {
        match value {
            ContainerType::Variant => zvariant::Signature::Variant,
            ContainerType::Array { value_type } => Self::array(value_type),
            ContainerType::Dict {
                key_type,
                value_type,
            } => Self::dict(key_type, value_type),
            ContainerType::Struct { fields } => {
                let field_signatures: Vec<zvariant::Signature> = fields
                    .into_iter()
                    .map(|f| Into::<zvariant::Signature>::into(f))
                    .collect();
                Self::Structure(field_signatures.into())
            }
        }
    }
}

impl From<ValueType> for zvariant::Signature {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::Primitive(t) => t.into(),
            ValueType::Container(t) => t.into(),
        }
    }
}

impl TryFrom<Array> for zvariant::Array<'static> {
    type Error = Error;

    fn try_from(value: Array) -> Result<Self, Self::Error> {
        Ok(match value {
            Array::ValueType(value_type) => Self::new(&value_type.into()),
            Array::Value(value) => {
                let value_type = value
                    .first()
                    .map(|f| f.type_())
                    .expect("Array::ValueType must be used for empty arrays");
                let mut array = Self::new(&value_type.into());
                for value in value {
                    array
                        .append(value.try_into()?)
                        .map_err(|_| Error::ElementsTypeIsDifferent)?;
                }
                array
            }
        })
    }
}

impl TryFrom<Dict> for zvariant::Dict<'static, 'static> {
    type Error = Error;

    fn try_from(value: Dict) -> Result<Self, Self::Error> {
        Ok(match value {
            Dict::ValueType {
                key_type,
                value_type,
            } => Self::new(&(*key_type).into(), &(*value_type).into()),
            Dict::Value { value } => {
                let value_type = value.values().next().map(|v| v.type_()).unwrap_or_default();
                let mut dict = Self::new(&zvariant::Signature::Str, &value_type.into());
                for (k, v) in value {
                    dict.append(zvariant::Value::Str(k.into()), v.try_into()?)
                        .map_err(|_| Error::ElementsTypeIsDifferent)?
                }
                dict
            }
        })
    }
}

impl From<PrimitiveValue> for zvariant::Value<'static> {
    fn from(value: PrimitiveValue) -> Self {
        match value {
            PrimitiveValue::U8(value) => value.into(),
            PrimitiveValue::Bool(value) => value.into(),
            PrimitiveValue::I16(value) => value.into(),
            PrimitiveValue::U16(value) => value.into(),
            PrimitiveValue::I32(value) => value.into(),
            PrimitiveValue::U32(value) => value.into(),
            PrimitiveValue::I64(value) => value.into(),
            PrimitiveValue::U64(value) => value.into(),
            PrimitiveValue::F64(value) => Self::F64(value.to_f64().unwrap_or_default()),
            PrimitiveValue::String(value) => value.into(),
            PrimitiveValue::Signature(value) => value.into(),
            PrimitiveValue::ObjectPath(value) => value.into(),
            PrimitiveValue::Fd(value) => Self::Fd(value.into()),
        }
    }
}

impl TryFrom<ContainerValue> for zvariant::Value<'static> {
    type Error = Error;

    fn try_from(value: ContainerValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ContainerValue::Variant { value } => Self::Value(Box::new((*value).try_into()?)),
            ContainerValue::Array(array) => Self::Array(array.try_into()?),
            ContainerValue::Dict(dict) => Self::Dict(dict.try_into()?),
            ContainerValue::Struct { value } => {
                let mut builder = zvariant::StructureBuilder::new();
                for v in value {
                    builder.push_value(v.try_into()?);
                }
                Self::Structure(builder.build().map_err(|_| Error::EmptyStructure)?)
            }
        })
    }
}

impl TryFrom<Value> for zvariant::Value<'static> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(match value {
            Value::Primitive(p) => p.into(),
            Value::Container(c) => c.try_into()?,
        })
    }
}

pub fn try_structure_from_fields(
    fields: impl IntoIterator<Item = Value>,
) -> Result<zvariant::Structure<'static>, Error> {
    let mut builder = zvariant::StructureBuilder::new();
    for value in fields {
        builder.push_value(value.try_into()?);
    }
    Ok(builder.build().map_err(|_| Error::EmptyStructure)?)
}

// zbus -> value
impl<'a> From<&'a zvariant::Signature> for PrimitiveType {
    fn from(value: &'a zvariant::Signature) -> Self {
        match value {
            zvariant::Signature::U8 => PrimitiveType::U8,
            zvariant::Signature::Bool => PrimitiveType::Bool,
            zvariant::Signature::I16 => PrimitiveType::I16,
            zvariant::Signature::U16 => PrimitiveType::U16,
            zvariant::Signature::I32 => PrimitiveType::I32,
            zvariant::Signature::U32 => PrimitiveType::U32,
            zvariant::Signature::I64 => PrimitiveType::I64,
            zvariant::Signature::U64 => PrimitiveType::U64,
            zvariant::Signature::F64 => PrimitiveType::F64,
            zvariant::Signature::Str => PrimitiveType::String,
            zvariant::Signature::Signature => PrimitiveType::Signature,
            zvariant::Signature::ObjectPath => PrimitiveType::ObjectPath,
            zvariant::Signature::Fd => PrimitiveType::Fd,
            _ => panic!("The type {:?} is not primitive", value),
        }
    }
}

impl<'a> From<&'a zvariant::Signature> for ValueType {
    fn from(value: &'a zvariant::Signature) -> Self {
        match value {
            zvariant::Signature::Unit => {
                panic!("Unit type is for internal use and should not be converted to the value")
            }
            zvariant::Signature::U8
            | zvariant::Signature::Bool
            | zvariant::Signature::I16
            | zvariant::Signature::U16
            | zvariant::Signature::I32
            | zvariant::Signature::U32
            | zvariant::Signature::I64
            | zvariant::Signature::U64
            | zvariant::Signature::F64
            | zvariant::Signature::Str
            | zvariant::Signature::Signature
            | zvariant::Signature::ObjectPath
            | zvariant::Signature::Fd => Self::Primitive(value.into()),
            zvariant::Signature::Variant => Self::Container(ContainerType::Variant),
            zvariant::Signature::Array(array) => Self::Container(ContainerType::Array {
                value_type: Box::new(array.signature().into()),
            }),
            zvariant::Signature::Dict { key, value } => Self::Container(ContainerType::Dict {
                key_type: Box::new(key.signature().into()),
                value_type: Box::new(value.signature().into()),
            }),
            zvariant::Signature::Structure(fields) => {
                let mut types: Vec<ValueType> = Vec::with_capacity(fields.len());
                for field in fields.iter() {
                    types.push(field.into());
                }
                Self::Container(ContainerType::Struct { fields: types })
            }
            #[cfg(feature = "gvariant")]
            zvariant::Signature::Maybe(_) => panic!("Type Maybe is not supported"),
        }
    }
}

impl<'a> From<&'a zvariant::Array<'a>> for Array {
    fn from(value: &'a zvariant::Array<'a>) -> Self {
        if value.is_empty() {
            Self::ValueType(value.element_signature().into())
        } else {
            Self::Value(value.iter().map(|v| v.into()).collect())
        }
    }
}

impl<'a> From<&'a zvariant::Dict<'a, 'a>> for Dict {
    fn from(value: &'a zvariant::Dict<'a, 'a>) -> Self {
        let map: HashMap<String, Value> = value
            .iter()
            .map(|(k, v)| (k.to_string(), v.into()))
            .collect();
        if map.is_empty() {
            let zvariant::Signature::Dict { key, value } = value.signature() else {
                panic!("Unexpected dict signature: {}", value.signature());
            };
            Self::ValueType {
                key_type: Box::new(key.signature().into()),
                value_type: Box::new(value.signature().into()),
            }
        } else {
            Self::Value { value: map }
        }
    }
}

impl<'a> From<&'a zvariant::Value<'a>> for Value {
    fn from(value: &'a zvariant::Value<'a>) -> Self {
        match value {
            zvariant::Value::U8(v) => Self::Primitive(PrimitiveValue::U8(*v)),
            zvariant::Value::Bool(v) => Self::Primitive(PrimitiveValue::Bool(*v)),
            zvariant::Value::I16(v) => Self::Primitive(PrimitiveValue::I16(*v)),
            zvariant::Value::U16(v) => Self::Primitive(PrimitiveValue::U16(*v)),
            zvariant::Value::I32(v) => Self::Primitive(PrimitiveValue::I32(*v)),
            zvariant::Value::U32(v) => Self::Primitive(PrimitiveValue::U32(*v)),
            zvariant::Value::I64(v) => Self::Primitive(PrimitiveValue::I64(*v)),
            zvariant::Value::U64(v) => Self::Primitive(PrimitiveValue::U64(*v)),
            zvariant::Value::F64(v) => Self::Primitive(PrimitiveValue::F64(
                Decimal::from_f64(*v).unwrap_or_default(),
            )),
            zvariant::Value::Str(v) => Self::Primitive(PrimitiveValue::String(v.to_string())),
            zvariant::Value::Signature(v) => Self::Primitive(PrimitiveValue::Signature(v.clone())),
            zvariant::Value::ObjectPath(v) => {
                Self::Primitive(PrimitiveValue::ObjectPath(v.clone().into()))
            }
            zvariant::Value::Value(v) => Self::Container(ContainerValue::Variant {
                value: Box::new(v.deref().into()),
            }),
            zvariant::Value::Array(v) => Self::Container(ContainerValue::Array(v.into())),
            zvariant::Value::Dict(v) => Self::Container(ContainerValue::Dict(v.into())),
            zvariant::Value::Structure(v) => Self::Container(ContainerValue::Struct {
                value: v.fields().iter().map(|f| f.into()).collect(),
            }),
            #[cfg(feature = "gvariant")]
            zvariant::Value::Maybe(v) => panic!("Type Maybe is not supported"),
            zvariant::Value::Fd(v) => Self::Primitive(PrimitiveValue::Fd(zvariant::OwnedFd::from(
                v.try_to_owned().unwrap(),
            ))),
        }
    }
}

impl<'a> From<zvariant::Value<'a>> for Value {
    fn from(value: zvariant::Value<'a>) -> Self {
        match value {
            zvariant::Value::U8(v) => Self::Primitive(PrimitiveValue::U8(v)),
            zvariant::Value::Bool(v) => Self::Primitive(PrimitiveValue::Bool(v)),
            zvariant::Value::I16(v) => Self::Primitive(PrimitiveValue::I16(v)),
            zvariant::Value::U16(v) => Self::Primitive(PrimitiveValue::U16(v)),
            zvariant::Value::I32(v) => Self::Primitive(PrimitiveValue::I32(v)),
            zvariant::Value::U32(v) => Self::Primitive(PrimitiveValue::U32(v)),
            zvariant::Value::I64(v) => Self::Primitive(PrimitiveValue::I64(v)),
            zvariant::Value::U64(v) => Self::Primitive(PrimitiveValue::U64(v)),
            zvariant::Value::F64(v) => Self::Primitive(PrimitiveValue::F64(
                Decimal::from_f64(v).unwrap_or_default(),
            )),
            zvariant::Value::Str(v) => Self::Primitive(PrimitiveValue::String(v.to_string())),
            zvariant::Value::Signature(v) => Self::Primitive(PrimitiveValue::Signature(v)),
            zvariant::Value::ObjectPath(v) => Self::Primitive(PrimitiveValue::ObjectPath(v.into())),
            zvariant::Value::Value(v) => Self::Container(ContainerValue::Variant {
                value: Box::new(v.deref().into()),
            }),
            zvariant::Value::Array(v) => Self::Container(ContainerValue::Array((&v).into())),
            zvariant::Value::Dict(v) => Self::Container(ContainerValue::Dict((&v).into())),
            zvariant::Value::Structure(v) => Self::Container(ContainerValue::Struct {
                value: v.fields().iter().map(|f| f.into()).collect(),
            }),
            #[cfg(feature = "gvariant")]
            zvariant::Value::Maybe(v) => panic!("Type Maybe is not supported"),
            zvariant::Value::Fd(v) => Self::Primitive(PrimitiveValue::Fd(zvariant::OwnedFd::from(
                v.try_to_owned().unwrap(),
            ))),
        }
    }
}

impl From<zvariant::OwnedValue> for Value {
    fn from(value: OwnedValue) -> Self {
        value.deref().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_primitive() {
        fn assert_json(primitive: PrimitiveValue, json: &str) {
            let serialized = serde_json::to_string(&Value::Primitive(primitive)).unwrap();
            let expected = json.to_string();
            assert_eq!(serialized, expected);
        }

        assert_json(PrimitiveValue::U8(12), r#"{"type":"u8","value":12}"#);
        assert_json(
            PrimitiveValue::Bool(false),
            r#"{"type":"bool","value":false}"#,
        );
        assert_json(PrimitiveValue::I16(-123), r#"{"type":"i16","value":-123}"#);
        assert_json(PrimitiveValue::U16(555), r#"{"type":"u16","value":555}"#);
        assert_json(
            PrimitiveValue::I32(-98765),
            r#"{"type":"i32","value":-98765}"#,
        );
        assert_json(
            PrimitiveValue::U32(56789),
            r#"{"type":"u32","value":56789}"#,
        );
        assert_json(
            PrimitiveValue::I64(-987654321),
            r#"{"type":"i64","value":-987654321}"#,
        );
        assert_json(
            PrimitiveValue::U64(123456789),
            r#"{"type":"u64","value":123456789}"#,
        );
        assert_json(
            PrimitiveValue::F64(Decimal::new(123456789, 3)),
            r#"{"type":"f64","value":123456.789}"#,
        );
        assert_json(
            PrimitiveValue::String("test_value".to_string()),
            r#"{"type":"string","value":"test_value"}"#,
        );
        assert_json(
            PrimitiveValue::Signature("(ais)".try_into().unwrap()),
            r#"{"type":"signature","value":"(ais)"}"#,
        );
        assert_json(
            PrimitiveValue::ObjectPath("/org/kde/ScreenBrightness/display12".try_into().unwrap()),
            r#"{"type":"objectPath","value":"/org/kde/ScreenBrightness/display12"}"#,
        );
    }

    #[test]
    fn serialize_variant() {
        let value: Value = ContainerValue::Variant {
            value: Box::new(PrimitiveValue::I32(123).into()),
        }
        .into();
        let json = r#"{"type":"variant","value":{"type":"i32","value":123}}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());
    }

    #[test]
    fn serialize_array() {
        let value = Value::Container(ContainerValue::Array(
            Array::ValueType(ValueType::default()),
        ));
        let json = r#"{"type":"array","valueType":"variant"}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());

        let value = Value::Container(ContainerValue::Array(Array::ValueType(
            ContainerType::Array {
                value_type: Box::new(PrimitiveType::String.into()),
            }
            .into(),
        )));
        let json = r#"{"type":"array","valueType":{"array":{"valueType":"string"}}}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());

        let value = Value::Container(ContainerValue::Array(Array::Value(vec![
            PrimitiveValue::I32(1).into(),
            PrimitiveValue::I32(2).into(),
            PrimitiveValue::I32(3).into(),
        ])));
        let json = r#"{"type":"array","value":[{"type":"i32","value":1},{"type":"i32","value":2},{"type":"i32","value":3}]}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());
    }

    #[test]
    fn serialize_dict() {
        let value: Value = ContainerValue::Dict(Dict::ValueType {
            key_type: Box::new(PrimitiveType::String),
            value_type: Box::new(PrimitiveType::I32.into()),
        })
        .into();
        let json = r#"{"type":"dict","keyType":"string","valueType":"i32"}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());

        let value: Value = ContainerValue::Dict(Dict::Value {
            value: HashMap::from([("key1".into(), PrimitiveValue::I32(123).into())]),
        })
        .into();
        let json = r#"{"type":"dict","value":{"key1":{"type":"i32","value":123}}}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());
    }

    #[test]
    fn serialize_struct() {
        let value: Value = ContainerValue::Struct {
            value: vec![
                PrimitiveValue::I32(123).into(),
                PrimitiveValue::String("s1".into()).into(),
                Value::Container(ContainerValue::Array(Array::Value(vec![
                    PrimitiveValue::I32(5).into(),
                    PrimitiveValue::I32(6).into(),
                    PrimitiveValue::I32(7).into(),
                ]))),
            ],
        }
        .into();
        let json = r#"{"type":"struct","value":[{"type":"i32","value":123},{"type":"string","value":"s1"},{"type":"array","value":[{"type":"i32","value":5},{"type":"i32","value":6},{"type":"i32","value":7}]}]}"#;
        assert_eq!(serde_json::to_string(&value).unwrap(), json.to_string());
    }
}
