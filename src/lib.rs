use naga::{front::wgsl, valid::Validator, Binding, StructMember, Type, TypeInner};
use wgpu::{VertexAttribute, VertexFormat};


pub mod gradient_triangle;

// pub fn to_vertex_attribute(ty: &Type, binding: Binding) -> VertexAttribute {
//     let vertex_attribute = match ty.inner {
//         TypeInner::Vector { size, scalar } => {
//             let vertex_format = match size {
//                 naga::VectorSize::Bi => match scalar.kind {
//                     naga::ScalarKind::Sint => VertexFormat::Sint16x2,
//                     naga::ScalarKind::Uint => VertexFormat::Uint16x2,
//                     naga::ScalarKind::Float => VertexFormat::Float16x2,
//                     naga::ScalarKind::Bool => todo!(),
//                     naga::ScalarKind::AbstractInt => todo!(),
//                     naga::ScalarKind::AbstractFloat => todo!(),
//                 },
//                 naga::VectorSize::Tri => match scalar.kind {
//                     naga::ScalarKind::Sint => VertexFormat::Sint32x3,
//                     naga::ScalarKind::Uint => VertexFormat::Uint32x3,
//                     naga::ScalarKind::Float => VertexFormat::Float32x3,
//                     naga::ScalarKind::Bool => todo!(),
//                     naga::ScalarKind::AbstractInt => todo!(),
//                     naga::ScalarKind::AbstractFloat => todo!(),
//                 },
//                 naga::VectorSize::Quad => match scalar.kind {
//                     naga::ScalarKind::Sint => VertexFormat::Sint32x4,
//                     naga::ScalarKind::Uint => VertexFormat::Uint32x4,
//                     naga::ScalarKind::Float => VertexFormat::Float32x4,
//                     naga::ScalarKind::Bool => todo!(),
//                     naga::ScalarKind::AbstractInt => todo!(),
//                     naga::ScalarKind::AbstractFloat => todo!(),
//                 },
//             };
//             VertexAttribute {
//                 format: vertex_format,
//                 // //offset: 
//                 shader_location: match binding {
//                     Binding::BuiltIn(built_in) => todo!(),
//                     Binding::Location { location, second_blend_source, interpolation, sampling } => todo!(),
//                 }
//             }
//         },
//         _ => todo!()
//     };
//     vertex_attribute
// }

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
                //entry.name += "_AMMENDTEST";
                //println!("function arguements \n {:#?}", entry.function.arguments);
            }
            //outputs edited wgsl

            let mut wgsl_out = String::new();
            let flags = naga::back::wgsl::WriterFlags::empty();
            let mut writer = naga::back::wgsl::Writer::new(&mut wgsl_out, flags);
            writer.write(&module, &module_info).unwrap();
            //fs::write("src/wgsl_out.wgsl", wgsl_out);
            println!("naga module: {:#?}", module);
            // let mut clipboard = clippers::Clipboard::get();
            // let _ = clipboard.write_text(format!("{:#?}", module));
            let layout = module.types.iter()
            // .filter_map(|(handle, ty)| {
            //     match ty.inner {
            //         TypeInner::Struct { members, span } => {
            //             let buffers = Vec::new();
            //             for member in members {
            //                 let ty = module.types.get_handle(member.ty).unwrap();

                            
            //                 VertexAttribute {
            //                     format: member
            //                 }
            //             }
            //         },
            //         _ => None
            //     }
            // })
            // .map(|ty| {
;
            // })
        }
        Err(err) => {
            panic!("SHADER INVALID: Reason: {:#?}", err);
        }
    }
}
