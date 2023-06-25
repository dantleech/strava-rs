use tui::widgets::TableState;

pub fn table_state_next(table_state: &mut TableState, max: usize) {

    let i = match table_state.selected() {
        Some(i) => {
            if i >= max - 1 {
                i
            } else {
                i + 1
            }
        },
        None => 0,
    };
    table_state.select(Some(i));
}

pub fn table_state_prev(table_state: &mut TableState, _max: usize) {
    let i = match table_state.selected() {
        Some(i) => {
            if i > 0 {
                i - 1
            } else {
                0
            }
        }
        None => 0,
    };
    table_state.select(Some(i));
}
