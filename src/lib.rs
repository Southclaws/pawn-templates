#[macro_use]
extern crate samp_sdk;
extern crate liquid;

mod plugin;
use plugin::Templates;

new_plugin!(Templates);
