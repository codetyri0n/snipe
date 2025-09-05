use arrow::datatypes::{Datatype, Field, Schema};
use arrow_array::{Array, ArrayRef};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub struct ArrowTypes;

impl ArrowTypes {
    #[inline]
    pub fn boolean_type() -> DataType {
        DataType::Boolean
    }
    #[inline]
    pub fn uint8_type() -> DataType {
        DataType::UInt8
    }
    #[inline]
    pub fn uint16_type() -> DataType {
        DataType::UInt16
    }
    #[inline]
    pub fn uint32_type() -> DataType {
        DataType::UInt32
    }
    #[inline]
    pub fn uint64_type() -> DataType {
        DataType::UInt64
    }
    #[inline]
    pub fn int8_type() -> DataType {
        DataType::Int8
    }
    #[inline]
    pub fn int16_type() -> DataType {
        DataType::Int16
    }
    #[inline]
    pub fn int32_type() -> DataType {
        DataType::Int32
    }
    #[inline]
    pub fn int64_type() -> DataType {
        DataType::Int64
    }
    #[inline]
    pub fn float32_type() -> DataType {
        DataType::Float32
    }
    #[inline]
    pub fn float64_type() -> DataType {
        DataType::Float64
    }
    #[inline]
    pub fn utf8_type() -> DataType {
        DataType::Utf8
    }
}

