mod bindings {
    windows::include_bindings!();
}

use bindings::{
    Windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK}
};

fn main() -> windows::Result<()> {
    unsafe {
        MessageBoxW(None, 
            "Hello, World!", 
            "Windows Rust Example", 
            MB_OK);
    }

    Ok(())
}
