use std::collections::HashMap;

pub fn get<P, V>(mut path: P, mut value: V) -> Result<V, PathError>
where
    P: Path,
    V: PathAccess<P::Component, Value = V>,
{
    loop {
        let (next_path, next_value) = path.step(value)?;

        path = next_path;
        value = next_value;

        if path.is_empty() {
            return Ok(value);
        }
    }
}

pub trait Path: Sized {
    type Component;

    fn is_empty(&self) -> bool;

    fn step<V>(self, value: V) -> Result<(Self, V::Value), PathError>
    where
        V: PathAccess<Self::Component>;
}

impl<'a> Path for &'a [Component<'a>] {
    type Component = Component<'a>;

    fn is_empty(&self) -> bool {
        <[_]>::is_empty(self)
    }

    fn step<V>(self, value: V) -> Result<(Self, V::Value), PathError>
    where
        V: PathAccess<Self::Component>,
    {
        if let Some((component, next)) = self.split_first() {
            value.get(component).map(|value| (next, value))
        } else {
            Err(PathError::None)
        }
    }
}

#[macro_export]
macro_rules! path {
    ($($part:expr),*) => {
        &[$(::rewr::path::Component::from($part),)*][..]
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Component<'a> {
    KeyInt(i64),
    KeyUInt(u64),
    KeyBytes(&'a [u8]),
    KeyString(&'a str),
    Index(usize),
}

impl<'a> From<&'a str> for Component<'a> {
    fn from(v: &'a str) -> Self {
        Self::KeyString(v)
    }
}

impl<'a> From<&'a [u8]> for Component<'a> {
    fn from(v: &'a [u8]) -> Self {
        Self::KeyBytes(v)
    }
}

impl<'a> From<[usize; 1]> for Component<'a> {
    fn from(v: [usize; 1]) -> Self {
        Self::Index(v[0])
    }
}

impl<'a> From<i8> for Component<'a> {
    fn from(v: i8) -> Self {
        Self::KeyInt(v as _)
    }
}

impl<'a> From<i16> for Component<'a> {
    fn from(v: i16) -> Self {
        Self::KeyInt(v as _)
    }
}

impl<'a> From<i32> for Component<'a> {
    fn from(v: i32) -> Self {
        Self::KeyInt(v as _)
    }
}

impl<'a> From<i64> for Component<'a> {
    fn from(v: i64) -> Self {
        Self::KeyInt(v as _)
    }
}

impl<'a> From<u8> for Component<'a> {
    fn from(v: u8) -> Self {
        Self::KeyUInt(v as _)
    }
}

impl<'a> From<u16> for Component<'a> {
    fn from(v: u16) -> Self {
        Self::KeyUInt(v as _)
    }
}

impl<'a> From<u32> for Component<'a> {
    fn from(v: u32) -> Self {
        Self::KeyUInt(v as _)
    }
}

impl<'a> From<u64> for Component<'a> {
    fn from(v: u64) -> Self {
        Self::KeyUInt(v as _)
    }
}

pub enum PathError {
    None,
    Type,
    Message(&'static str),
}

pub trait PathAccess<A>: Sized {
    type Value;

    fn get(self, accessor: &A) -> Result<Self::Value, PathError>;
}

pub enum Value {
    Str(String),
    Integer(u64),
    Map(HashMap<String, Self>),
}

impl<'a, 'b> PathAccess<Component<'b>> for &'a Value {
    type Value = &'a Value;

    fn get(self, accessor: &Component<'b>) -> Result<Self::Value, PathError> {
        match (self, *accessor) {
            (Value::Map(ref map), Component::KeyString(k)) => map.get(k).ok_or(PathError::None),
            _ => Err(PathError::Type),
        }
    }
}
