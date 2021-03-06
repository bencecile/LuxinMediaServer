use std::{
    env,
    fs::{File},
    path::{Path},
};
use gl_generator::{Registry, Api, Profile, Fallbacks, StructGenerator};

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(Path::new(dest).join("gl_bindings.rs")).unwrap();

    Registry::new(Api::Gl, (3, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(StructGenerator, &mut file)
        .unwrap();
}
