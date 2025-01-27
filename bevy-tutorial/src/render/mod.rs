use crate::prelude::*;

/// Projects a position into screen bevy coordinates (0, 0) is on the middle of the screen.
/// Also adjusting for a sprite's offset.
pub fn project_pos_into_screen(point_offset: f32, window_length: f32, sprite_offset: f32) -> f32 {
    let without_offset = point_offset - (window_length / 2.0);
    return without_offset + (-without_offset.signum() * sprite_offset);
}

/// Checks if a point is confined within the window boundaries and returns a bitwise flag
/// indicating its confinement status.
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

    confinement_status: ConfinementStatus,
}

type ConfinementStatus = u8;

impl Confinement {
    pub fn is_confined(&self) -> bool {
        return self.confinement_status == CONFINED;
    }

    pub fn is(&self, status: ConfinementStatus) -> bool {
        return self.confinement_status & status != 0;
    }
}

pub const CONFINED: ConfinementStatus = 0b0000;
pub const OUTSIDE_LEFT: ConfinementStatus = 0b0001;
pub const OUTSIDE_RIGHT: ConfinementStatus = 0b0010;
pub const OUTSIDE_TOP: ConfinementStatus = 0b0100;
pub const OUTSIDE_BOTTOM: ConfinementStatus = 0b1000;

fn window_limits(window_length: f32, sprite_offset: f32) -> (f32, f32) {
    let half_window = window_length / 2.0;
    return (-half_window + sprite_offset, half_window - sprite_offset);
}
