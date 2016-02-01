use std::any::Any;

pub trait State: Any {}

impl <T> State for T where T: Any {  }
