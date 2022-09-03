// use crate::{scene::EditorScene, GameEngine};
// use fyrox::{
//     core::{pool::Handle, scope_profile},
//     gui::{
//         button::{ButtonBuilder, ButtonMessage},
//         grid::{Column, GridBuilder, Row},
//         message::{MessageDirection, UiMessage},
//         numeric::{NumericUpDownBuilder, NumericUpDownMessage},
//         text::TextBuilder,
//         widget::WidgetBuilder,
//         window::{WindowBuilder, WindowTitle},
//         Thickness, UiNode, VerticalAlignment,
//     },
//     utils::lightmap::Lightmap,
// };

use ne_gui::{
    button::{ButtonBuilder, ButtonMessage},
    core::pool::Handle,
    prelude::{Column, GridBuilder, Row, UiMessage},
    widget::WidgetBuilder,
    window::{WindowBuilder, WindowTitle},
    Thickness, UiNode, NUserInterface
};
use bevy_ecs::world::World;
pub struct BasicWindow {
    pub window: Handle<UiNode>,
    // nud_texels_per_unit: Handle<UiNode>,
    // nud_spacing: Handle<UiNode>,
    button: Handle<UiNode>,
    // texels_per_unit: u32,
    // spacing: f32,
}

impl BasicWindow {
    pub fn new(world: &mut World) -> Self {
        let button;
        //do I have insert a mutable one?
        let mut gui = 
        world.get_non_send_resource_mut::<NUserInterface>().unwrap();
        let ctx = &mut gui.user_interface.build_ctx();

        let window = 
        WindowBuilder::new(WidgetBuilder::new().with_width(300.0).with_height(400.0))
            .with_title(WindowTitle::Text("Light Settings".to_owned()))
            .open(false)
            .with_content(
                GridBuilder::new(
                    WidgetBuilder::new()
                        .with_child({
                            button = ButtonBuilder::new(
                                WidgetBuilder::new()
                                    .on_row(2)
                                    .on_column(1)
                                    .with_margin(Thickness::uniform(1.0)),
                            )
                            .with_text("Generate Lightmap")
                            .build(ctx);
                            button
                        }),
                )
                .add_column(Column::strict(100.0))
                .add_column(Column::stretch())
                .add_row(Row::strict(25.0))
                .add_row(Row::strict(25.0))
                .add_row(Row::strict(25.0))
                .add_row(Row::stretch())

                //how to mutably borrow multiple times in the same scope? ...
                .build(ctx),
            )
            .build(ctx);
        Self { window, button }
    }
    //handle_ui_message is in every window hm?
    pub fn handle_ui_message(
        &mut self,
        message: &UiMessage,
        // editor_scene: &EditorScene,
        // engine: &mut GameEngine,
    ) {
        // scope_profile!();

        if let Some(ButtonMessage::Click) = message.data::<ButtonMessage>() {
            if message.destination() == self.button {
                println!("button pressed!");
            }
        }
    }
}
