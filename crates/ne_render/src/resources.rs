// use cfg_if::cfg_if;

/* pub async fn load_string(file_name: &str) -> anyhow::Result<String> {
    cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            let url = format_url(file_name);
            let txt = reqwest::get(url)
                .await?
                .text()
                .await?;
        } else {
            let s = env!("CARGO_MANIFEST_DIR").to_owned()+"/../../engine_assets";
            ne::log!("{}", s);
            let path = std::path::Path::new(&s)
                .join(file_name);

                ne::log!("{}", path.display());
            let txt = std::fs::read_to_string(path)?;
        }
    }

    Ok(txt)
} */
