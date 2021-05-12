// bindings\build.rs
fn main() {
    windows::build!(
      Windows::Win32::UI::WindowsAndMessaging::*,
    );
}
