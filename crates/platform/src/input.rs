use winit::event::VirtualKeyCode;

/// Variant enum for `Input` used to distinguish between input types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputVariant {
	Unknown,
	Key {
		virtual_code: VirtualKeyCode,
		symbol: char,
	},
	MouseButton(u8),
	MouseAxis,
}

/// Static information about input sets
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Input {
	pub display_name: &'static str,
	pub variant: InputVariant,
}

impl Input {
	const fn key(display_name: &'static str, virtual_code: VirtualKeyCode, symbol: char) -> Self {
		Self {
			display_name,
			variant: InputVariant::Key {
				virtual_code,
				symbol,
			},
		}
	}

	const fn mouse_button(display_name: &'static str, index: u8) -> Self {
		Self {
			display_name,
			variant: InputVariant::MouseButton(index),
		}
	}

	const fn mouse_axis(display_name: &'static str) -> Self {
		Self {
			display_name,
			variant: InputVariant::MouseAxis,
		}
	}

	/// Returns corresponding key `Input` based on a key code
	///
	/// # Arguments
	///
	/// * `in_code` - A u8 which is the unique key code
	///
	/// # Examples
	///
	/// ```
	/// use os::input::Input;
	/// let a_key = Input::key_from_code(0x41);
	/// ```
	///
	/// # Todo
	///
	/// * `(Speed)` - Use lookup table to speed up find.
	pub fn key_from_code(in_virtual_code: VirtualKeyCode) -> Option<Self> {
		for input in ALL_INPUTS.iter() {
			if let InputVariant::Key {
				virtual_code: code,
				symbol: _,
			} = input.variant
			{
				if in_virtual_code == code {
					return Some(*input);
				}
			}
		}

		None
	}

	pub fn as_key(self) -> (VirtualKeyCode, char) {
		match self.variant {
			InputVariant::Key {
				virtual_code: code,
				symbol,
			} => (code, symbol),
			_ => unreachable!(),
		}
	}

	pub fn as_mouse_button(self) -> u8 {
		match self.variant {
			InputVariant::MouseButton(code) => code,
			_ => unreachable!(),
		}
	}
}

pub const UNKNOWN: Input = Input {
	display_name: "Unknown",
	variant: InputVariant::Unknown,
};

pub const KEY_BACKSPACE: Input = Input::key("Backspace", VirtualKeyCode::Back, '\0');
pub const KEY_TAB: Input = Input::key("Tab", VirtualKeyCode::Tab, '\t');
pub const KEY_ENTER: Input = Input::key("Enter", VirtualKeyCode::Return, '\0');

pub const KEY_0: Input = Input::key("Zero Key", VirtualKeyCode::Key0, '0');
pub const KEY_1: Input = Input::key("One Key", VirtualKeyCode::Key1, '1');
pub const KEY_2: Input = Input::key("Two Key", VirtualKeyCode::Key2, '2');
pub const KEY_3: Input = Input::key("Three Key", VirtualKeyCode::Key3, '3');
pub const KEY_4: Input = Input::key("Four Key", VirtualKeyCode::Key4, '4');
pub const KEY_5: Input = Input::key("Five Key", VirtualKeyCode::Key5, '5');
pub const KEY_6: Input = Input::key("Six Key", VirtualKeyCode::Key6, '6');
pub const KEY_7: Input = Input::key("Seven Key", VirtualKeyCode::Key7, '7');
pub const KEY_8: Input = Input::key("Eight Key", VirtualKeyCode::Key8, '8');
pub const KEY_9: Input = Input::key("Nine Key", VirtualKeyCode::Key9, '9');

pub const KEY_A: Input = Input::key("A Key", VirtualKeyCode::A, 'A');
pub const KEY_B: Input = Input::key("B Key", VirtualKeyCode::B, 'B');
pub const KEY_C: Input = Input::key("C Key", VirtualKeyCode::C, 'C');
pub const KEY_D: Input = Input::key("D Key", VirtualKeyCode::D, 'D');
pub const KEY_E: Input = Input::key("E Key", VirtualKeyCode::E, 'E');
pub const KEY_F: Input = Input::key("F Key", VirtualKeyCode::F, 'F');
pub const KEY_G: Input = Input::key("G Key", VirtualKeyCode::G, 'G');
pub const KEY_H: Input = Input::key("H Key", VirtualKeyCode::H, 'H');
pub const KEY_I: Input = Input::key("I Key", VirtualKeyCode::I, 'I');
pub const KEY_J: Input = Input::key("J Key", VirtualKeyCode::J, 'J');
pub const KEY_K: Input = Input::key("K Key", VirtualKeyCode::K, 'K');
pub const KEY_L: Input = Input::key("L Key", VirtualKeyCode::L, 'L');
pub const KEY_M: Input = Input::key("M Key", VirtualKeyCode::M, 'M');
pub const KEY_N: Input = Input::key("N Key", VirtualKeyCode::N, 'N');
pub const KEY_O: Input = Input::key("O Key", VirtualKeyCode::O, 'O');
pub const KEY_P: Input = Input::key("P Key", VirtualKeyCode::P, 'P');
pub const KEY_Q: Input = Input::key("Q Key", VirtualKeyCode::Q, 'Q');
pub const KEY_R: Input = Input::key("R Key", VirtualKeyCode::R, 'R');
pub const KEY_S: Input = Input::key("S Key", VirtualKeyCode::S, 'S');
pub const KEY_T: Input = Input::key("T Key", VirtualKeyCode::T, 'T');
pub const KEY_U: Input = Input::key("U Key", VirtualKeyCode::U, 'U');
pub const KEY_V: Input = Input::key("V Key", VirtualKeyCode::V, 'V');
pub const KEY_W: Input = Input::key("W Key", VirtualKeyCode::W, 'W');
pub const KEY_X: Input = Input::key("X Key", VirtualKeyCode::X, 'X');
pub const KEY_Y: Input = Input::key("Y Key", VirtualKeyCode::Y, 'Y');
pub const KEY_Z: Input = Input::key("Z Key", VirtualKeyCode::Z, 'Z');

pub const KEY_ESCAPE: Input = Input::key("Escape Key", 0x1B, '\0');
pub const KEY_SHIFT: Input = Input::key("Shift Key", 0x10, '\0');
pub const KEY_CTRL: Input = Input::key("Ctrl Key", 0x11, '\0');
pub const KEY_ALT: Input = Input::key("Alt Key", 0x12, '\0');
pub const KEY_PAUSE: Input = Input::key("Pause Key", 0x13, '\0');
pub const KEY_CAPITAL: Input = Input::key("Capital Key", 0x14, '\0');
pub const KEY_SPACE: Input = Input::key("Space Key", 0x20, '\0');
pub const KEY_PRIOR: Input = Input::key("Prior Key", 0x21, '\0');
pub const KEY_NEXT: Input = Input::key("Next Key", 0x22, '\0');
pub const KEY_END: Input = Input::key("End Key", 0x23, '\0');
pub const KEY_HOME: Input = Input::key("Home Key", 0x24, '\0');
pub const KEY_LEFT: Input = Input::key("Left Key", 0x25, '\0');
pub const KEY_UP: Input = Input::key("Up Key", 0x26, '\0');
pub const KEY_RIGHT: Input = Input::key("Right Key", 0x27, '\0');
pub const KEY_DOWN: Input = Input::key("Down Key", 0x28, '\0');
pub const KEY_SELECT: Input = Input::key("Select Key", 0x29, '\0');
pub const KEY_PRINT: Input = Input::key("Print Key", 0x2A, '\0');
pub const KEY_SNAPSHOT: Input = Input::key("Snapshot Key", 0x2C, '\0');
pub const KEY_INSERT: Input = Input::key("Insert Key", 0x2D, '\0');
pub const KEY_DELETE: Input = Input::key("Delete Key", 0x2E, '\0');
pub const KEY_HELP: Input = Input::key("Help Key", 0x2F, '\0');

pub const KEY_NUMPAD0: Input = Input::key("Numpad 0 Key", 0x60, '0');
pub const KEY_NUMPAD1: Input = Input::key("Numpad 1 Key", 0x61, '1');
pub const KEY_NUMPAD2: Input = Input::key("Numpad 2 Key", 0x62, '2');
pub const KEY_NUMPAD3: Input = Input::key("Numpad 3 Key", 0x63, '3');
pub const KEY_NUMPAD4: Input = Input::key("Numpad 4 Key", 0x64, '4');
pub const KEY_NUMPAD5: Input = Input::key("Numpad 5 Key", 0x65, '5');
pub const KEY_NUMPAD6: Input = Input::key("Numpad 6 Key", 0x66, '6');
pub const KEY_NUMPAD7: Input = Input::key("Numpad 7 Key", 0x67, '7');
pub const KEY_NUMPAD8: Input = Input::key("Numpad 8 Key", 0x68, '8');
pub const KEY_NUMPAD9: Input = Input::key("Numpad 9 Key", 0x69, '9');
pub const KEY_MULTIPLY: Input = Input::key("* Key", 0x6A, '*');
pub const KEY_ADD: Input = Input::key("+ Key", 0x6B, '+');
pub const KEY_SEPARATOR: Input = Input::key("| Key", 0x6C, '|');
pub const KEY_SUBTRACT: Input = Input::key("- Key", 0x6D, '-');
pub const KEY_DECIMAL: Input = Input::key(". Key", 0x6E, '.');
pub const KEY_DIVIDE: Input = Input::key("/ Key", 0x6F, '/');
pub const KEY_F1: Input = Input::key("F1 Key", 0x70, '\0');
pub const KEY_F2: Input = Input::key("F2 Key", 0x71, '\0');
pub const KEY_F3: Input = Input::key("F3 Key", 0x72, '\0');
pub const KEY_F4: Input = Input::key("F4 Key", 0x73, '\0');
pub const KEY_F5: Input = Input::key("F5 Key", 0x74, '\0');
pub const KEY_F6: Input = Input::key("F6 Key", 0x75, '\0');
pub const KEY_F7: Input = Input::key("F7 Key", 0x76, '\0');
pub const KEY_F8: Input = Input::key("F8 Key", 0x77, '\0');
pub const KEY_F9: Input = Input::key("F9 Key", 0x78, '\0');
pub const KEY_F10: Input = Input::key("F10 Key", 0x79, '\0');
pub const KEY_F11: Input = Input::key("F11 Key", 0x7A, '\0');
pub const KEY_F12: Input = Input::key("F12 Key", 0x7B, '\0');
pub const KEY_F13: Input = Input::key("F13 Key", 0x7C, '\0');
pub const KEY_F14: Input = Input::key("F14 Key", 0x7D, '\0');
pub const KEY_F15: Input = Input::key("F15 Key", 0x7E, '\0');
pub const KEY_F16: Input = Input::key("F16 Key", 0x7F, '\0');
pub const KEY_F17: Input = Input::key("F17 Key", 0x80, '\0');
pub const KEY_F18: Input = Input::key("F18 Key", 0x81, '\0');
pub const KEY_F19: Input = Input::key("F19 Key", 0x82, '\0');
pub const KEY_F20: Input = Input::key("F20 Key", 0x83, '\0');
pub const KEY_F21: Input = Input::key("F21 Key", 0x84, '\0');
pub const KEY_F22: Input = Input::key("F22 Key", 0x85, '\0');
pub const KEY_F23: Input = Input::key("F23 Key", 0x86, '\0');
pub const KEY_F24: Input = Input::key("F24 Key", 0x87, '\0');

pub const KEY_NUMLOCK: Input = Input::key("Numlock Key", 0x90, '\0');
pub const KEY_SCROLL: Input = Input::key("Scroll Key", 0x91, '\0');

pub const KEY_LSHIFT: Input = Input::key("Left Shift Key", 0xA0, '\0');
pub const KEY_RSHIFT: Input = Input::key("Right Shift Key", 0xA1, '\0');
pub const KEY_LEFT_CTRL: Input = Input::key("Left Ctrl Key", 0xA2, '\0');
pub const KEY_RIGHT_CTRL: Input = Input::key("Right Ctrl", 0xA3, '\0');
pub const KEY_LEFT_MENU: Input = Input::key("Left Menu Key", 0xA4, '\0');
pub const KEY_RIGHT_RMENU: Input = Input::key("Right Menu Key", 0xA5, '\0');

pub const KEY_TILDE: Input = Input::key("Tilde Key", 0xC0, '~');

pub const MOUSE_BUTTON_LEFT: Input = Input::mouse_button("Left Mouse Button", 0);
pub const MOUSE_BUTTON_MIDDLE: Input = Input::mouse_button("Middle Mouse Button", 1);
pub const MOUSE_BUTTON_RIGHT: Input = Input::mouse_button("Right Mouse Button", 2);

pub const MOUSE_AXIS_X: Input = Input::mouse_axis("Mouse Axis X");
pub const MOUSE_AXIS_Y: Input = Input::mouse_axis("Mouse Axis Y");

pub const NUM_INPUTS: usize = 114;
pub const ALL_INPUTS: [Input; NUM_INPUTS] = [
	KEY_BACKSPACE,
	KEY_TAB,
	KEY_ENTER,
	KEY_0,
	KEY_1,
	KEY_2,
	KEY_3,
	KEY_4,
	KEY_5,
	KEY_6,
	KEY_7,
	KEY_8,
	KEY_9,
	KEY_A,
	KEY_B,
	KEY_C,
	KEY_D,
	KEY_E,
	KEY_F,
	KEY_G,
	KEY_H,
	KEY_I,
	KEY_J,
	KEY_K,
	KEY_L,
	KEY_M,
	KEY_N,
	KEY_O,
	KEY_P,
	KEY_Q,
	KEY_R,
	KEY_S,
	KEY_T,
	KEY_U,
	KEY_V,
	KEY_W,
	KEY_X,
	KEY_Y,
	KEY_Z,
	KEY_ESCAPE,
	KEY_SHIFT,
	KEY_CTRL,
	KEY_ALT,
	KEY_PAUSE,
	KEY_CAPITAL,
	KEY_SPACE,
	KEY_PRIOR,
	KEY_NEXT,
	KEY_END,
	KEY_HOME,
	KEY_LEFT,
	KEY_UP,
	KEY_RIGHT,
	KEY_DOWN,
	KEY_SELECT,
	KEY_PRINT,
	KEY_SNAPSHOT,
	KEY_INSERT,
	KEY_DELETE,
	KEY_HELP,
	KEY_NUMPAD0,
	KEY_NUMPAD1,
	KEY_NUMPAD2,
	KEY_NUMPAD3,
	KEY_NUMPAD4,
	KEY_NUMPAD5,
	KEY_NUMPAD6,
	KEY_NUMPAD7,
	KEY_NUMPAD8,
	KEY_NUMPAD9,
	KEY_MULTIPLY,
	KEY_ADD,
	KEY_SEPARATOR,
	KEY_SUBTRACT,
	KEY_DECIMAL,
	KEY_DIVIDE,
	KEY_F1,
	KEY_F2,
	KEY_F3,
	KEY_F4,
	KEY_F5,
	KEY_F6,
	KEY_F7,
	KEY_F8,
	KEY_F9,
	KEY_F10,
	KEY_F11,
	KEY_F12,
	KEY_F13,
	KEY_F14,
	KEY_F15,
	KEY_F16,
	KEY_F17,
	KEY_F18,
	KEY_F19,
	KEY_F20,
	KEY_F21,
	KEY_F22,
	KEY_F23,
	KEY_F24,
	KEY_NUMLOCK,
	KEY_SCROLL,
	KEY_LSHIFT,
	KEY_RSHIFT,
	KEY_LEFT_CTRL,
	KEY_RIGHT_CTRL,
	KEY_LEFT_MENU,
	KEY_RIGHT_RMENU,
	KEY_TILDE,
	MOUSE_BUTTON_LEFT,
	MOUSE_BUTTON_MIDDLE,
	MOUSE_BUTTON_RIGHT,
	MOUSE_AXIS_X,
	MOUSE_AXIS_Y,
];
