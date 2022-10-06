#![feature(stmt_expr_attributes)]
// env::CARGO_MANIFEST_DIR;

// TODO I want this as a string literal
// const asset_path:

// debug only
///example
///let x = find_file!("C:/git/tools/nightmare_engine", "/Cargo.toml");
///ne::log!("{}", x)
///Finds and returns a file, will throw error if file is not available
///$path either absolute or relative from your_crate/src
#[macro_export]
macro_rules! find_file {
    ($path:literal) => {{
    //opportunity for improvement
    #[cfg(debug_assertions)]
        {
            let _ = include_bytes!($path);
            let r = $path;
            r
        }
    }};
    ($path1:literal,$path2:literal) => {{
        #[cfg(debug_assertions)]
        {
            let _ = include_bytes!(concat!($path1, $path2));
            let r = concat!($path1, $path2);
            r
        }
    }};
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
