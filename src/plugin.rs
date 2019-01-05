extern crate liquid;

use samp_sdk::amx::AMX;
use samp_sdk::consts::*;
use samp_sdk::types::Cell;

pub struct Templates{
    //
}

define_native!(create_template, template: String);
define_native!(render_template, id: Cell);

impl Templates {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {
        return;
    }

    pub fn amx_load(&self, amx: &AMX) -> Cell {
        return AMX_ERR_NONE;
    }

    pub fn amx_unload(&self, amx: &AMX) -> Cell {
        return AMX_ERR_NONE;
    }

    pub fn create_template(&mut self, _: &AMX, template: String) -> Cell {
        // let parser = liquid::ParserBuilder::with_liquid().build().unwrap();

        // let template = parser.parse(&template);
  
        return 0;
    }
 
    pub fn render_template(&mut self, _: &AMX, id: Cell) -> Cell {
        return id;
    }
}


impl Default for Templates {
    fn default() -> Self {
        Templates {
        }
    }
}