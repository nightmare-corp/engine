use std::{env, path::PathBuf};
use ne::*;
//honestly not sure how to implement this...
pub fn get_exec_dir() -> String
{
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    cwd + "/"
}
pub fn get_asset_dir() -> String
{
    get_exec_dir()+"assets/"
}
pub fn get_shaders_dir() -> String
{
    get_asset_dir()+"shaders/"
}
pub fn get_shader(shader_name:&str) -> String
{
    get_shaders_dir()+shader_name
}