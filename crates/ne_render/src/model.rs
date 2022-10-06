// /// * `path:String` is relative path from engine_assets... TODO FOR NOW, so path = "shapes/cube.obj" => "\engine_assets\shapes\cube.obj"
// /// * `location:Vec3` is location in world space.
// pub struct ModelDescriptor {
//     pub path:String, //optimize for multiple meshes.
//     pub transform:Transform,
// }

// use crate::{mesh::Mesh, material::Material};
// ///Model holds multiple meshes and multiple materials.
// pub struct Model {
//     meshes:Vec<Mesh>,
//     materials: Vec<Material>,
// }
// impl Model {
//     pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>) -> Self { 
//         Self {
//             meshes, materials
//         }
//     }
// }