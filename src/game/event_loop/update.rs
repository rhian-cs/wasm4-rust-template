use crate::engine::input;
use crate::game::state::STATE;

use super::{draw, FrameState};

pub unsafe fn update() {
    let frame_state = FrameState {
        mouse_x: input::mouse_x() as u8,
        mouse_y: input::mouse_y() as u8,
    };

    STATE.update(frame_state);

    draw(&STATE);
}
