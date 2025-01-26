use crate::prelude::*;

/// Projects a position into screen bevy coordinates (0, 0) is on the middle of the screen.
/// Also adjusting for a sprite's offset.
pub fn project_pos_into_screen(point_offset: f32, window_length: f32, sprite_offset: f32) -> f32 {
    let without_offset = point_offset - (window_length / 2.0);
    return without_offset + (-without_offset.signum() * sprite_offset);
}
/// Checks if a point is confined within the window boundaries and returns a bitwise flag
/// indicating its confinement status.
///
/// # Returns
/// A `u8` value representing the confinement status as a combination of bitwise flags.
/// See the constants `render::CONFINED`, `render::OUTSIDE_LEFT`, `render::OUTSIDE_RIGHT`,
/// `render::OUTSIDE_TOP`, `render::OUTSIDE_BOTTOM`
///
/// # Example
/// ```
/// let mut translation = Vec3::new(-10.0, 500.0, 0.0);
/// let window = Window::new(800.0, 600.0);
/// let sprite_offset = 32.0;
///
/// let status = is_confined(&mut translation, &window, sprite_offset);
///
/// if status == render::CONFINED {
///     println!("The point is confined within the window.");
/// } else {
///     if status & render::OUTSIDE_LEFT != 0 {
///         println!("The point is outside the left boundary.");
///     }
/// }
/// ```
pub fn check_confinement(translation: &Vec3, window: &Window, sprite_offset: f32) -> Confinement {
    let (x_min, x_max) = window_limits(window.width(), sprite_offset);
    let (y_min, y_max) = window_limits(window.height(), sprite_offset);

    let mut confinement_status = CONFINED;

    if translation.x < x_min {
        confinement_status |= OUTSIDE_LEFT;
    } else if translation.x > x_max {
        confinement_status |= OUTSIDE_RIGHT;
    }

    if translation.y < y_min {
        confinement_status |= OUTSIDE_BOTTOM;
    } else if translation.y > y_max {
        confinement_status |= OUTSIDE_TOP;
    }

    return Confinement {
        x_min,
        x_max,
        y_min,
        y_max,
        confinement_status,
    };
}

pub struct Confinement {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,

    confinement_status: u8,
}

impl Confinement {
    pub fn is_confined(&self) -> bool {
        return self.confinement_status == CONFINED;
    }

    pub fn is_outside_left(&self) -> bool {
        return self.confinement_status & OUTSIDE_LEFT != 0;
    }

    pub fn is_outside_right(&self) -> bool {
        return self.confinement_status & OUTSIDE_RIGHT != 0;
    }

    pub fn is_outside_bottom(&self) -> bool {
        return self.confinement_status & OUTSIDE_BOTTOM != 0;
    }

    pub fn is_outside_top(&self) -> bool {
        return self.confinement_status & OUTSIDE_TOP != 0;
    }
}

pub const CONFINED: u8 = 0b0000;
pub const OUTSIDE_LEFT: u8 = 0b0001;
pub const OUTSIDE_RIGHT: u8 = 0b0010;
pub const OUTSIDE_TOP: u8 = 0b0100;
pub const OUTSIDE_BOTTOM: u8 = 0b1000;

fn window_limits(window_length: f32, sprite_offset: f32) -> (f32, f32) {
    let half_window = window_length / 2.0;
    return (-half_window + sprite_offset, half_window - sprite_offset);
}
