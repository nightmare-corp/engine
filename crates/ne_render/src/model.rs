// /// * `path:String` is relative path from engine_assets... TODO FOR NOW, so path = "shapes/cube.obj" => "\engine_assets\shapes\cube.obj"
// /// * `location:Vec3` is location in world space.
pub struct ModelDescriptor {
    pub path:String, //optimize for multiple meshes.
    pub transform:Transform,
}
