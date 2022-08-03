// env::CARGO_MANIFEST_DIR;


///returns literal path to assets dir
#[macro_export]
macro_rules! get_assets_dir{
    () =>
    {
        // find_file!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets")
        concat!(env!("CARGO_MANIFEST_DIR"), "../assets")
    }
}


// TODO CFG only. To decrease compile time. 
///example 
///let x = find_file!("C:/git/tools/nightmare_engine", "/Cargo.toml");
///println!("{}", x)
///Finds and returns a file, will throw error if file is not available
#[macro_export]
macro_rules! find_file{
    ($arg1:literal) => {
        {
            //this doesn't work eh? 
            if cfg!(feature="path_checker") 
            {
                println!("path_checker");
                let _ = include_bytes!($arg1);
            }
            let r = $arg1;
            r
        }

        //TODO
        // #[cfg(not(feature="path_checker"))]
        // {
        //     r
        // }
    };
    ($arg1:literal,$arg2:literal) => {
        #[cfg(feature="path_checker")]
        { 
            if cfg!(feature="path_checker") 
            {
                let _ = include_bytes!(concat!($arg1,$arg2));
            }
            let r = concat!($arg1,$arg2);
            r
        }
    };
}

#[macro_export]
macro_rules! into_str2 {
    ($arg1:literal,$arg2:literal) => {
        { 
            let s = into_str!(concat!($arg1,$arg2));
            s
        }
    };
}
#[macro_export]
macro_rules! into_byte2 {
    ($arg1:literal,$arg2:literal) => {
        { 
            let b = into_byte2!(concat!($arg1,$arg2));
            b
        }
    };
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
