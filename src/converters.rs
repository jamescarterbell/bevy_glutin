use bevy_input::{
    keyboard::{ElementState, KeyCode, KeyboardInput},
    mouse::MouseButton,
};

pub fn convert_keyboard_input(keyboard_input: &glutin::event::KeyboardInput) -> KeyboardInput {
    KeyboardInput {
        scan_code: keyboard_input.scancode,
        state: convert_element_state(keyboard_input.state),
        key_code: keyboard_input.virtual_keycode.map(convert_virtual_key_code),
    }
}

pub fn convert_element_state(element_state: glutin::event::ElementState) -> ElementState {
    match element_state {
        glutin::event::ElementState::Pressed => ElementState::Pressed,
        glutin::event::ElementState::Released => ElementState::Released,
    }
}

pub fn convert_mouse_button(mouse_button: glutin::event::MouseButton) -> MouseButton {
    match mouse_button {
        glutin::event::MouseButton::Left => MouseButton::Left,
        glutin::event::MouseButton::Right => MouseButton::Right,
        glutin::event::MouseButton::Middle => MouseButton::Middle,
        glutin::event::MouseButton::Other(val) => MouseButton::Other(val),
    }
}

pub fn convert_virtual_key_code(virtual_key_code: glutin::event::VirtualKeyCode) -> KeyCode {
    match virtual_key_code {
        glutin::event::VirtualKeyCode::Key1 => KeyCode::Key1,
        glutin::event::VirtualKeyCode::Key2 => KeyCode::Key2,
        glutin::event::VirtualKeyCode::Key3 => KeyCode::Key3,
        glutin::event::VirtualKeyCode::Key4 => KeyCode::Key4,
        glutin::event::VirtualKeyCode::Key5 => KeyCode::Key5,
        glutin::event::VirtualKeyCode::Key6 => KeyCode::Key6,
        glutin::event::VirtualKeyCode::Key7 => KeyCode::Key7,
        glutin::event::VirtualKeyCode::Key8 => KeyCode::Key8,
        glutin::event::VirtualKeyCode::Key9 => KeyCode::Key9,
        glutin::event::VirtualKeyCode::Key0 => KeyCode::Key0,
        glutin::event::VirtualKeyCode::A => KeyCode::A,
        glutin::event::VirtualKeyCode::B => KeyCode::B,
        glutin::event::VirtualKeyCode::C => KeyCode::C,
        glutin::event::VirtualKeyCode::D => KeyCode::D,
        glutin::event::VirtualKeyCode::E => KeyCode::E,
        glutin::event::VirtualKeyCode::F => KeyCode::F,
        glutin::event::VirtualKeyCode::G => KeyCode::G,
        glutin::event::VirtualKeyCode::H => KeyCode::H,
        glutin::event::VirtualKeyCode::I => KeyCode::I,
        glutin::event::VirtualKeyCode::J => KeyCode::J,
        glutin::event::VirtualKeyCode::K => KeyCode::K,
        glutin::event::VirtualKeyCode::L => KeyCode::L,
        glutin::event::VirtualKeyCode::M => KeyCode::M,
        glutin::event::VirtualKeyCode::N => KeyCode::N,
        glutin::event::VirtualKeyCode::O => KeyCode::O,
        glutin::event::VirtualKeyCode::P => KeyCode::P,
        glutin::event::VirtualKeyCode::Q => KeyCode::Q,
        glutin::event::VirtualKeyCode::R => KeyCode::R,
        glutin::event::VirtualKeyCode::S => KeyCode::S,
        glutin::event::VirtualKeyCode::T => KeyCode::T,
        glutin::event::VirtualKeyCode::U => KeyCode::U,
        glutin::event::VirtualKeyCode::V => KeyCode::V,
        glutin::event::VirtualKeyCode::W => KeyCode::W,
        glutin::event::VirtualKeyCode::X => KeyCode::X,
        glutin::event::VirtualKeyCode::Y => KeyCode::Y,
        glutin::event::VirtualKeyCode::Z => KeyCode::Z,
        glutin::event::VirtualKeyCode::Escape => KeyCode::Escape,
        glutin::event::VirtualKeyCode::F1 => KeyCode::F1,
        glutin::event::VirtualKeyCode::F2 => KeyCode::F2,
        glutin::event::VirtualKeyCode::F3 => KeyCode::F3,
        glutin::event::VirtualKeyCode::F4 => KeyCode::F4,
        glutin::event::VirtualKeyCode::F5 => KeyCode::F5,
        glutin::event::VirtualKeyCode::F6 => KeyCode::F6,
        glutin::event::VirtualKeyCode::F7 => KeyCode::F7,
        glutin::event::VirtualKeyCode::F8 => KeyCode::F8,
        glutin::event::VirtualKeyCode::F9 => KeyCode::F9,
        glutin::event::VirtualKeyCode::F10 => KeyCode::F10,
        glutin::event::VirtualKeyCode::F11 => KeyCode::F11,
        glutin::event::VirtualKeyCode::F12 => KeyCode::F12,
        glutin::event::VirtualKeyCode::F13 => KeyCode::F13,
        glutin::event::VirtualKeyCode::F14 => KeyCode::F14,
        glutin::event::VirtualKeyCode::F15 => KeyCode::F15,
        glutin::event::VirtualKeyCode::F16 => KeyCode::F16,
        glutin::event::VirtualKeyCode::F17 => KeyCode::F17,
        glutin::event::VirtualKeyCode::F18 => KeyCode::F18,
        glutin::event::VirtualKeyCode::F19 => KeyCode::F19,
        glutin::event::VirtualKeyCode::F20 => KeyCode::F20,
        glutin::event::VirtualKeyCode::F21 => KeyCode::F21,
        glutin::event::VirtualKeyCode::F22 => KeyCode::F22,
        glutin::event::VirtualKeyCode::F23 => KeyCode::F23,
        glutin::event::VirtualKeyCode::F24 => KeyCode::F24,
        glutin::event::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
        glutin::event::VirtualKeyCode::Scroll => KeyCode::Scroll,
        glutin::event::VirtualKeyCode::Pause => KeyCode::Pause,
        glutin::event::VirtualKeyCode::Insert => KeyCode::Insert,
        glutin::event::VirtualKeyCode::Home => KeyCode::Home,
        glutin::event::VirtualKeyCode::Delete => KeyCode::Delete,
        glutin::event::VirtualKeyCode::End => KeyCode::End,
        glutin::event::VirtualKeyCode::PageDown => KeyCode::PageDown,
        glutin::event::VirtualKeyCode::PageUp => KeyCode::PageUp,
        glutin::event::VirtualKeyCode::Left => KeyCode::Left,
        glutin::event::VirtualKeyCode::Up => KeyCode::Up,
        glutin::event::VirtualKeyCode::Right => KeyCode::Right,
        glutin::event::VirtualKeyCode::Down => KeyCode::Down,
        glutin::event::VirtualKeyCode::Back => KeyCode::Back,
        glutin::event::VirtualKeyCode::Return => KeyCode::Return,
        glutin::event::VirtualKeyCode::Space => KeyCode::Space,
        glutin::event::VirtualKeyCode::Compose => KeyCode::Compose,
        glutin::event::VirtualKeyCode::Caret => KeyCode::Caret,
        glutin::event::VirtualKeyCode::Numlock => KeyCode::Numlock,
        glutin::event::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
        glutin::event::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
        glutin::event::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
        glutin::event::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
        glutin::event::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
        glutin::event::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
        glutin::event::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
        glutin::event::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
        glutin::event::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
        glutin::event::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
        glutin::event::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
        glutin::event::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
        glutin::event::VirtualKeyCode::Add => KeyCode::Add,
        glutin::event::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
        glutin::event::VirtualKeyCode::Apps => KeyCode::Apps,
        glutin::event::VirtualKeyCode::At => KeyCode::At,
        glutin::event::VirtualKeyCode::Ax => KeyCode::Ax,
        glutin::event::VirtualKeyCode::Backslash => KeyCode::Backslash,
        glutin::event::VirtualKeyCode::Calculator => KeyCode::Calculator,
        glutin::event::VirtualKeyCode::Capital => KeyCode::Capital,
        glutin::event::VirtualKeyCode::Colon => KeyCode::Colon,
        glutin::event::VirtualKeyCode::Comma => KeyCode::Comma,
        glutin::event::VirtualKeyCode::Convert => KeyCode::Convert,
        glutin::event::VirtualKeyCode::Decimal => KeyCode::Decimal,
        glutin::event::VirtualKeyCode::Divide => KeyCode::Divide,
        glutin::event::VirtualKeyCode::Equals => KeyCode::Equals,
        glutin::event::VirtualKeyCode::Grave => KeyCode::Grave,
        glutin::event::VirtualKeyCode::Kana => KeyCode::Kana,
        glutin::event::VirtualKeyCode::Kanji => KeyCode::Kanji,
        glutin::event::VirtualKeyCode::LAlt => KeyCode::LAlt,
        glutin::event::VirtualKeyCode::LBracket => KeyCode::LBracket,
        glutin::event::VirtualKeyCode::LControl => KeyCode::LControl,
        glutin::event::VirtualKeyCode::LShift => KeyCode::LShift,
        glutin::event::VirtualKeyCode::LWin => KeyCode::LWin,
        glutin::event::VirtualKeyCode::Mail => KeyCode::Mail,
        glutin::event::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
        glutin::event::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
        glutin::event::VirtualKeyCode::Minus => KeyCode::Minus,
        glutin::event::VirtualKeyCode::Multiply => KeyCode::Multiply,
        glutin::event::VirtualKeyCode::Mute => KeyCode::Mute,
        glutin::event::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
        glutin::event::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
        glutin::event::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
        glutin::event::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
        glutin::event::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
        glutin::event::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
        glutin::event::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
        glutin::event::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
        glutin::event::VirtualKeyCode::OEM102 => KeyCode::OEM102,
        glutin::event::VirtualKeyCode::Period => KeyCode::Period,
        glutin::event::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
        glutin::event::VirtualKeyCode::Power => KeyCode::Power,
        glutin::event::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
        glutin::event::VirtualKeyCode::RAlt => KeyCode::RAlt,
        glutin::event::VirtualKeyCode::RBracket => KeyCode::RBracket,
        glutin::event::VirtualKeyCode::RControl => KeyCode::RControl,
        glutin::event::VirtualKeyCode::RShift => KeyCode::RShift,
        glutin::event::VirtualKeyCode::RWin => KeyCode::RWin,
        glutin::event::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
        glutin::event::VirtualKeyCode::Slash => KeyCode::Slash,
        glutin::event::VirtualKeyCode::Sleep => KeyCode::Sleep,
        glutin::event::VirtualKeyCode::Stop => KeyCode::Stop,
        glutin::event::VirtualKeyCode::Subtract => KeyCode::Subtract,
        glutin::event::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
        glutin::event::VirtualKeyCode::Tab => KeyCode::Tab,
        glutin::event::VirtualKeyCode::Underline => KeyCode::Underline,
        glutin::event::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
        glutin::event::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
        glutin::event::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
        glutin::event::VirtualKeyCode::Wake => KeyCode::Wake,
        glutin::event::VirtualKeyCode::WebBack => KeyCode::WebBack,
        glutin::event::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
        glutin::event::VirtualKeyCode::WebForward => KeyCode::WebForward,
        glutin::event::VirtualKeyCode::WebHome => KeyCode::WebHome,
        glutin::event::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
        glutin::event::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
        glutin::event::VirtualKeyCode::WebStop => KeyCode::WebStop,
        glutin::event::VirtualKeyCode::Yen => KeyCode::Yen,
        glutin::event::VirtualKeyCode::Copy => KeyCode::Copy,
        glutin::event::VirtualKeyCode::Paste => KeyCode::Paste,
        glutin::event::VirtualKeyCode::Cut => KeyCode::Cut,
    }
}
