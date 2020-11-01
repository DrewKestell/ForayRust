use crate::ui::Layer;

pub enum Event {
    ChangeActiveLayer{ layer: Layer },
    GamepadInput{ input_value: u16, modifier: i16, pressed: bool },
    KeyDown{ char_code: u64 },
    KeyUp{ char_code: u64 },
    MouseMove{ mouse_pos_x: f32, mouse_pos_y: f32 },
    LeftMouseDown{ mouse_pos_x: f32, mouse_pos_y: f32 },
    MiddleMouseDown{ mouse_pos_x: f32, mouse_pos_y: f32 },
    RightMouseDown{ mouse_pos_x: f32, mouse_pos_y: f32 },
    LeftMouseUp{ mouse_pos_x: f32, mouse_pos_y: f32 },
    MiddleMouseUp{ mouse_pos_x: f32, mouse_pos_y: f32 },
    RightMouseUp{ mouse_pos_x: f32, mouse_pos_y: f32 },
}