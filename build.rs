extern crate gl_generator;

use std::fs::File;
use std::path::Path;
use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};

fn main() {
    let mut file = File::create(Path::new("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gl, (2, 0), Profile::Core, Fallbacks::All, []).write_bindings(GlobalGenerator, &mut file).unwrap();
}