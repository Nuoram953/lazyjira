use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{ActiveList, App};

pub fn draw(f: &mut Frame, app: &mut App) {
    let (left_rows, right_col) = main_chunks(f.area());

    app.items_sprint
        .draw(f, left_rows[0], app.navigator.active == ActiveList::Sprint);

    app.items_backlog
        .draw(f, left_rows[1], app.navigator.active == ActiveList::Backlog);

    app.items_recently_updated.draw(
        f,
        left_rows[2],
        app.navigator.active == ActiveList::RecentlyUpdated,
    );

    f.render_widget(
        Block::default().title("Details").borders(Borders::ALL),
        right_col,
    );
}

pub fn main_chunks(area: Rect) -> (Vec<Rect>, Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let left_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(columns[0])
        .to_vec();

    let right_col = columns[1];

    (left_rows, right_col)
}
