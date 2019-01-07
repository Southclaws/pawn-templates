use samp_sdk::amx::{AmxResult, AMX};
use samp_sdk::args;
use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use std::collections::HashMap;

pub struct Templates {
    pool: HashMap<Cell, liquid::Template>,
    id: Cell,
}

define_native!(create_template, template: String);
define_native!(render_template as raw);

#[derive(Debug)]
enum ArgumentPairType {
    Invalid = 0,
    String = 1,
    Int = 2,
    Float = 3,
}

impl ArgumentPairType {
    fn from_i32(i: i32) -> ArgumentPairType {
        match i {
            1 => ArgumentPairType::String,
            2 => ArgumentPairType::Int,
            3 => ArgumentPairType::Float,
            _ => ArgumentPairType::Invalid,
        }
    }
}

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

    pub fn render_template(&mut self, amx: &AMX, params: *mut Cell) -> AmxResult<Cell> {
        let varargc = args_count!(params) - 3;
        let pairs = match varargc > 0 && varargc % 3 == 0 {
            true => varargc / 3,
            false => {
                log!("Invalid number of arguments passed to RenderTemplate");
                return Ok(1);
            }
        };

        let mut parser = args::Parser::new(params);
        expand_args!(@amx, parser, template_id: Cell);
        expand_args!(@amx, parser, output_string_amx: Cell);
        let output_string = amx.get_address(output_string_amx)?;
        expand_args!(@amx, parser, output_length: usize);

        log!(
            "Template ID: {}, Output dest: {:p}, Output length: {}",
            template_id,
            output_string,
            output_length
        );

        let t = match self.pool.get(&template_id) {
            Some(t) => t,
            None => return Ok(1),
        };

        let mut variables = liquid::value::Object::new();

        for _ in 0..pairs {
            let mut pair_type: Cell = 0;
            get_arg_ref(amx, &mut parser, &mut pair_type);

            let mut key = String::new();
            get_arg_string(amx, &mut parser, &mut key);

            match ArgumentPairType::from_i32(pair_type) {
                ArgumentPairType::String => {
                    log!("Type is string");

                    let mut val = String::new();
                    get_arg_string(amx, &mut parser, &mut val);

                    variables.insert(key.into(), liquid::value::Value::scalar(val));
                }
                ArgumentPairType::Int => {
                    log!("Type is int");

                    let mut val: Cell = 0;
                    get_arg_ref(amx, &mut parser, &mut val);

                    variables.insert(key.into(), liquid::value::Value::scalar(val));
                }
                ArgumentPairType::Float => {
                    log!("Type is float");

                    let mut val: f32 = 0.0;
                    get_arg_ref(amx, &mut parser, &mut val);

                    variables.insert(key.into(), liquid::value::Value::scalar(val as f64));
                }
                _ => {
                    log!("Type is unknown");
                }
            };
        }

        let output = match t.render(&variables) {
            Ok(v) => v,
            Err(e) => {
                log!("{}", e);
                return Ok(1);
            }
        };

        log!("Rendered output: {}", output);

        let encoded = samp_sdk::cp1251::encode(&output)?;
        set_string!(encoded, output_string, output_length);

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

fn get_arg_ref<T: Clone>(amx: &AMX, parser: &mut args::Parser, out_ref: &mut T) -> i32 {
    expand_args!(@amx, parser, tmp_ref: ref T);
    *out_ref = tmp_ref.clone();
    return 1;
}

fn get_arg_string(amx: &AMX, parser: &mut args::Parser, out_str: &mut String) -> i32 {
    expand_args!(@amx, parser, tmp_str: String);
    match samp_sdk::cp1251::decode_to(&tmp_str.into_bytes(), out_str) {
        Ok(_) => {
            return 1;
        },
        Err(e) => {
            log!("{}", e);
            return 0;
        }
    }
}
