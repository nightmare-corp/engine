/// Thank you people at Fyrox
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        brush::Brush,
        dock::{DockingManagerBuilder, TileBuilder, TileContent},
        draw,
        dropdown_list::DropdownListBuilder,
        file_browser::{FileBrowserMode, FileSelectorBuilder, FileSelectorMessage, Filter},
        formatted_text::WrapMode,
        grid::{Column, GridBuilder, Row},
        message::{KeyCode, MessageDirection, UiMessage},
        messagebox::{MessageBoxBuilder, MessageBoxButtons, MessageBoxMessage, MessageBoxResult},
        ttf::Font,
        widget::{WidgetBuilder, WidgetMessage},
        window::{WindowBuilder, WindowMessage, WindowTitle},
        BuildContext, UiNode, UserInterface, VerticalAlignment,
    };
}
pub use fyrox_ui::*;


pub struct NUserInterface
{
    //pub?
    pub user_interface:UserInterface,
}
impl NUserInterface
{
    pub fn new(width:f32,height:f32)  -> Self
    {
        println!("{}", &width);
        println!("{}", &height);

        //remove nalgebra...
        Self{
            user_interface: UserInterface::new(
                nalgebra::Vector2::new(width,height))
        }
    }
}