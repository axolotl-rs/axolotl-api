use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::ser::State;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use crate::item::Item;
use crate::world::{BlockPosition, GenericLocation, World, WorldLocation};
use crate::world_gen::noise::ChunkGenerator;
use crate::NameSpaceRef;

/// A Generic Block State Type
#[derive(Debug, Clone, PartialEq)]
pub enum BlockStateValue {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}
pub struct BlockStateVisitor;
impl<'de> Visitor<'de> for BlockStateVisitor {
    type Value = BlockStateValue;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("BlockStateValue")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Bool(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(BlockStateValue::Int(v as i32))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::Float(v as f32))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(BlockStateValue::String(v))
    }
}
impl<'de> Deserialize<'de> for BlockStateValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(BlockStateVisitor)
    }
}

pub trait BlockState: Debug {
    fn get(&self, name: &str) -> Option<&BlockStateValue>;

    fn set(&mut self, name: impl Into<String>, value: BlockStateValue);
}

pub trait Block: Item {
    type State: BlockState;

    fn get_default_state(&self) -> Self::State;
}

impl<B> Block for &'_ B
where
    B: Block,
{
    type State = B::State;

    fn get_default_state(&self) -> Self::State {
        (*self).get_default_state()
    }
}
