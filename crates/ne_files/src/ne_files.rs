// env::CARGO_MANIFEST_DIR;

// TODO I want this as a string literal
// const asset_path:

///returns literal path to assets dir
/// TODO
#[macro_export]
macro_rules! find_asset {
    //returns asset dir
    () => {
        {
            //TODO this needs to generate code from outisde text file?
            //I need to somehow get the absolute path of asset dir..?
            // "/home/karlot/projects/nightmare_engine/assets"

            //TODO I also want the binary executable in the main project.


            // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
            println!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets"));
            concat!(env!("CARGO_MANIFEST_DIR"), "/../assets")
        }
    };
    //path from asset dir
    //next step is to remove the /../ and add a assets dir besides cargo_manifest...
    //if it is the same for each crate.
    () => {
        {
            // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
            println!(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets"));
            concat!(env!("CARGO_MANIFEST_DIR"), "/../assets")
        }
    };
}

// TODO CFG only. To decrease compile time.
///example
///let x = find_file!("C:/git/tools/nightmare_engine", "/Cargo.toml");
///println!("{}", x)
///Finds and returns a file, will throw error if file is not available
///$path either absolute or relative from your_crate/src
#[macro_export]
macro_rules! find_file {
    ($path:literal) => {
        {
            //opportunity for improvement
            let _ = include_bytes!($path);
            let r = $path;
            r
        }
    };

    //TODO
    // #[cfg(not(feature="path_checker"))]
    // {
    //     r
    // }};
    ($path1:literal,$path2:literal) => {
        #[cfg(feature = "path_checker")]
        {
            let _ = include_bytes!(concat!($path1, $path2));
            let r = concat!($path1, $path2);
            r
        }
    };
}

#[macro_export]
macro_rules! into_str2 {
    ($arg1:literal,$arg2:literal) => {{
        let s = into_str!(concat!($arg1, $arg2));
        s
    }};
}
#[macro_export]
macro_rules! into_byte2 {
    ($arg1:literal,$arg2:literal) => {{
        let b = into_byte2!(concat!($arg1, $arg2));
        b
    }};
}

// #[proc_macro]
// pub fn my_proc_macro(input: TokenStream) -> TokenStream {
//     let s = "C:\\git\\tools\\nightmare_engine";
//     // assert!(s=="C:\\git\\tools\\nightmare_engine");

//     let path = Path::new(&s);

//     assert!(path.exists());

//     let out:TokenStream = s.parse().unwrap();

//     out
// }
