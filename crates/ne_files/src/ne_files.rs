use std::{env, path::PathBuf};
use ne::*;
//honestly not sure how to implement this...
pub fn GetExecDir() -> String
{
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    // L::error!(cwd);
    cwd
    // match path 
    // {
    //     PathBuf => PathBuf.into_os_string().into_string(),
    //     Error => {
    //         L::warn!("Initialized logging [WARN]");
    //         ""
    //     }
    // }
}

pub fn GetAssets() -> &'static str
{

    ""
}

