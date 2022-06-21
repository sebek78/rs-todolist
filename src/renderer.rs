use crate::state::AppState;

pub fn render(state: &AppState) {
    println!("{}", state.menu_index);
}
