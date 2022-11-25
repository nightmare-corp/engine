///this function prints the name and size of specified struct
pub fn print_size_of<T>() {
    let name = std::any::type_name::<T>();
    let size = std::intrinsics::size_of::<T>();
    println!("type: {}, size: {}", name, size);
}
// type: i8, size: 1
// type: i32, size: 4
// type: i64, size: 8
// type: i128, size: 16
// type: &str, size: 16
// type: alloc::string::String, size: 24
// type: alloc::vec::Vec<alloc::string::String>, size: 24