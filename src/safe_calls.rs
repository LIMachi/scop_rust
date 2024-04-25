use gl::{Clear, ClearColor, COLOR_BUFFER_BIT};

pub fn clear_screen() {
    unsafe {
        Clear(COLOR_BUFFER_BIT);
    }
}

pub fn set_clear_color(red: f32, green: f32, blue: f32) {
    unsafe {
        ClearColor(red, green, blue, 1.0);
    }
}