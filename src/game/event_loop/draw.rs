use crate::engine::external_functions::rect;
use crate::game::point::Point;
use crate::game::state::State;

pub unsafe fn draw(state: &State) {
    example_draw(&state);
}

unsafe fn example_draw(state: &State) {
    draw_point(&state.mouse_pointer);
}

unsafe fn draw_point(point: &Point) {
    rect((point.x - 2) as u32, (point.y - 2) as u32, 5, 5);
}
