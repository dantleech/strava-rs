use tui::widgets::TableState;

pub fn table_state_next(table_state: &mut TableState, max: usize, repeat: bool) {
    let i = match table_state.selected() {
        Some(i) => {
            if max == 0 {
                return;
            }
            if i >= max - 1 {
                if repeat { 0 } else { i }
            } else {
                i + 1
            }
        }
        None => 0,
    };
    table_state.select(Some(i));
}

pub fn table_state_prev(table_state: &mut TableState, max: usize, repeat: bool) {
    let i = match table_state.selected() {
        Some(i) => {
            if i > 0 {
                i - 1
            } else {
                if repeat { max - 1 } else { 0 }
            }
        }
        None => 0,
    };
    table_state.select(Some(i));
}
