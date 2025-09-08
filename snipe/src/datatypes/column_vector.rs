use crate::datatypes::arrowtypes::ArrowTypes;
use std::any::*;

trait ColumnVector {
    fn get_type(&self) -> ArrowTypes;
    fn get_value(&self, i : usize) -> Option<Box<dyn Any>>;
    fn size(&self) -> usize;
}