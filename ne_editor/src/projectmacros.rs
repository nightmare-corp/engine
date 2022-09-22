///returns literal path to assets dir
#[macro_export]
macro_rules! find_asset {
    //returns asset dir
    () => {{
        //TODO this needs to generate code from outisde text file?
        //I need to somehow get the absolute path of asset dir..?
        // "/home/karlot/projects/nightmare_engine/assets"

        //TODO I also want the binary executable in the main project.

        // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        ne::log!(concat!(env!("CARGO_MANIFEST_DIR"), "/../engine_assets"));
        concat!(env!("CARGO_MANIFEST_DIR"), "/../engine_assets")
    }};
    //path from asset dir
    //next step is to remove the /../ and add a assets dir besides cargo_manifest...
    //if it is the same for each crate.
    () => {{
        // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        ne::log!(find_asset!(), "/../engine_assets");
        concat!(find_asset!(), "/../engine_assets")
    }};
}
