use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MESSAGEBOX_STYLE};

fn main() {
    let mb = MESSAGEBOX_STYLE(1);
    unsafe {
        let result = MessageBoxA(None, "Text", "Title", mb);
        println!("Result {:?}", result);
    }
}
