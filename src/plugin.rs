use samp_sdk::amx::{AmxResult, AMX};
use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use std::collections::HashMap;

pub struct Templates {
    pool: HashMap<Cell, liquid::Template>,
    id: Cell,
}

define_native!(create_template, template: String);
define_native!(render_template, id: Cell, dest: ref Cell);

impl Templates {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {
        return;
    }

    pub fn amx_load(&self, amx: &AMX) -> Cell {
        let natives = natives! {
            "CreateTemplate" => create_template,
            "RenderTemplate" => render_template
        };

        match amx.register(&natives) {
            Ok(_) => AMX_ERR_NONE,
            Err(err) => {
                log!("failed to register natives: {:?}", err);
                AMX_ERR_INIT
            }
        }
    }

    pub fn amx_unload(&self, _: &AMX) -> Cell {
        return AMX_ERR_NONE;
    }

    pub fn create_template(&mut self, _: &AMX, template: String) -> AmxResult<Cell> {
        let id = self.alloc();

        let parser = liquid::ParserBuilder::with_liquid().build().unwrap();

        let t = match parser.parse(&template) {
            Ok(v) => v,
            Err(e) => {
                log!("{}", e);
                return Ok(1);
            }
        };
        self.pool.insert(id, t);

        return Ok(id);
    }

    pub fn render_template(&mut self, _: &AMX, id: Cell, dest: &mut Cell) -> AmxResult<Cell> {
        let t = match self.pool.get(&id) {
            Some(t) => t,
            None => return Ok(1),
        };

        let mut variables = liquid::value::Object::new();

        // TODO: read variadics and transform into variables
        variables.insert("name".into(), liquid::value::Value::scalar("Southclaws"));

        let output = match t.render(&variables) {
            Ok(v) => v,
            Err(e) => {
                log!("{}", e);
                return Ok(1);
            }
        };

        let s = String::into_bytes(output);

        set_string!(s, dest, s.len());

        return Ok(0);
    }

    fn alloc(&mut self) -> Cell {
        self.id += 1;
        return self.id;
    }
}

impl Default for Templates {
    fn default() -> Self {
        Templates {
            pool: HashMap::new(),
            id: 0,
        }
    }
}
