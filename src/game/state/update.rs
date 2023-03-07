use crate::game::event_loop::FrameState;
use crate::game::Point;

use super::State;

impl State {
    pub fn update(&mut self, frame_state: FrameState) {
        self.mouse_pointer = Point {
            x: frame_state.mouse_x,
            y: frame_state.mouse_y,
        };
    }
}
