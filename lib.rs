extern crate runtime;

use runtime::runtime::runtime_types::Context;
use runtime::runtime::runtime_types::PointerTypes;
use runtime::runtime::runtime_types::PublicData;
use runtime::runtime::runtime_types::Types;
use runtime::runtime::runtime_types::*;
use runtime::runtime::*;

pub struct string {
    
}

impl runtime::runtime::Library for string {
    fn call(
        &mut self,
        id: usize,
        mem: PublicData,
    ) -> Result<runtime_types::Types, runtime_error::ErrTypes> {
        match id {
            0 => {
                
            }
            _ => {
                unreachable!("Invalid function id")
            }
        }
        return Ok(runtime_types::Types::Null);
    }
    fn name(&self) -> String {
        return "string".to_owned();
    }
    fn register(&self) -> Vec<(String, usize)> {
        return vec![];
    }
}

#[no_mangle]
pub fn init(ctx: &mut Context) -> Box<dyn Library> {
    return Box::new(string {  });
}
