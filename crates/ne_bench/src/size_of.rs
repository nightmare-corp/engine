
///this function prints the name and size of specified struct
pub fn print_size_of<T>()
{
    let name =  std::any::type_name::<T>();
    let size = std::intrinsics::size_of::<T>();
    println!("type: {}, size: {}", name, size);
}