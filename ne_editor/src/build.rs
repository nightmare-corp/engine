use std::env;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;

    Ok(())
}