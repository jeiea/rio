use rio_window::event::ElementState;
use rio_window::platform::windows::WindowsKeyData;
use std::io::Write;

/// ConPTY's DEC mode 9001 carries one KEY_EVENT_RECORD per UTF-16 code unit.
pub fn encode(native: WindowsKeyData, state: ElementState, text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut record = |unicode: u16| {
        write!(
            bytes,
            "\x1b[{};{};{};{};{};{}_",
            native.virtual_key,
            native.scan_code,
            unicode,
            u8::from(state == ElementState::Pressed),
            native.control_key_state,
            native.repeat_count
        )
        .expect("writing to Vec");
    };
    if state == ElementState::Released {
        record(native.release_code_unit());
    } else if text.is_empty() {
        record(0);
    } else {
        text.encode_utf16().for_each(record);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win32_records_distinguish_ctrl_slash_from_ctrl_shift_minus() {
        let slash = WindowsKeyData {
            virtual_key: 191,
            scan_code: 53,
            control_key_state: 8,
            repeat_count: 1,
        };
        let minus = WindowsKeyData {
            virtual_key: 189,
            scan_code: 12,
            control_key_state: 24,
            repeat_count: 3,
        };
        assert_eq!(
            encode(slash, ElementState::Pressed, ""),
            b"\x1b[191;53;0;1;8;1_"
        );
        assert_eq!(
            encode(minus, ElementState::Pressed, "\x1f"),
            b"\x1b[189;12;31;1;24;3_"
        );
    }

    #[test]
    fn enter_keeps_its_character_on_release() {
        let enter = WindowsKeyData {
            virtual_key: 13,
            scan_code: 28,
            control_key_state: 0,
            repeat_count: 1,
        };
        assert_eq!(
            encode(enter, ElementState::Released, ""),
            b"\x1b[13;28;13;0;0;1_"
        );
    }
}
