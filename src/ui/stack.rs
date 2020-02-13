mod raw;
mod tree;

use super::super::ui;

pub fn setup(state: &ui::State) {
    tree::setup(&state);
    raw::setup(&state);
}

pub fn disable(state: &ui::State) {
    tree::disable(state);
    raw::disable(state);
}

pub fn enable(state: &ui::State) {
    tree::enable(state);
    raw::enable(state);
}

pub fn redisplay_data(state: &ui::State) {
    tree::redisplay_data(&state);
    raw::redisplay_data(&state);
}
