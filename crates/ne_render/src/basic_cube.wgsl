struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

@group(0)
@binding(0)
var<uniform> model_matrix: mat4x4<f32>;
@group(0)
@binding(1)
var<uniform> view_projection_matrix: mat4x4<f32>; //or is it view_world/world_view 

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
) -> VertexOutput {
    let world_position = model_matrix * position;

    var result: VertexOutput;
    // result.clip_position = view_projection_matrix * world_position;
    result.clip_position = world_position;
    result.tex_coord = tex_coord;
    return result;
}

@group(0)
@binding(2)
var r_color: texture_2d<u32>;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    let v = f32(tex.x) / 255.0;
    return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}
