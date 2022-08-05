
#[allow(non_camel_case_types)]
pub struct nightmare_engine
{
}
//TODO Is this needed?
// impl Default for nightmare_engine
// {
//     // fn default() -> Self {
//     // }
// }
impl nightmare_engine
{
    pub fn new() -> nightmare_engine
    {
        // nightmare_engine::default()
        nightmare_engine::empty()
    }
    pub fn empty() -> nightmare_engine {
        Self {
        }
    }
    pub fn add_plugin(self) -> nightmare_engine {

        self
    }
}