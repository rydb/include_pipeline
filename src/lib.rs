use naga::{front::wgsl, valid::Validator};


pub mod gradient_triangle;


pub fn parse_wgsl_shader(shader_as_str: &str) {
    let mut module = wgsl::parse_str(shader_as_str).unwrap();

    let validation_results = Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module);
    match validation_results {
        Ok(module_info) => {
            for entry in module.entry_points.iter_mut() {
                //println!("entry point: {:#} \n {:#?}", entry.name, entry.function);
                entry.name += "_AMMENDTEST";
                //println!("function arguements \n {:#?}", entry.function.arguments);
            }
            //outputs edited wgsl

            let mut wgsl_out = String::new();
            let flags = naga::back::wgsl::WriterFlags::empty();
            let mut writer = naga::back::wgsl::Writer::new(&mut wgsl_out, flags);
            writer.write(&module, &module_info).unwrap();
            //fs::write("src/wgsl_out.wgsl", wgsl_out);
        }
        Err(err) => {
            panic!("SHADER INVALID: Reason: {:#?}", err);
        }
    }
}
