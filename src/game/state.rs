use super::Point;

mod update;

pub struct State {
    pub mouse_pointer: Point,
}

pub static mut STATE: State = State {
    mouse_pointer: Point { x: 0, y: 0 },
};
