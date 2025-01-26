pub fn window_limits(window_length: f32, sprite_offset: f32) -> (f32, f32) {
    let half_window = window_length / 2.0;
    return (-half_window + sprite_offset, half_window - sprite_offset);
}

pub fn project_into_screen(point_offset: f32, window_length: f32, sprite_offset: f32) -> f32 {
    let without_offset = point_offset - (window_length / 2.0);
    return without_offset + (-without_offset.signum() * sprite_offset);
}
