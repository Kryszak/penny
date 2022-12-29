use super::{app_state::FileEntry, App};
use std::vec;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use tui_logger::TuiLoggerWidget;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_view_constraint = Constraint::Max(80);
    let logs_view_constraint = match app.state.logs_visible {
        true => Constraint::Percentage(20),
        false => Constraint::Length(0),
    };
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([main_view_constraint, logs_view_constraint].as_ref())
        .split(f.size());

    if app.state.logs_visible {
        f.render_widget(draw_log_view(), chunks[1]);
    }

    render_main_view(f, chunks[0], app);
}

fn render_main_view<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let help_constraint = match app.state.help_visible {
        true => Constraint::Percentage(15),
        false => Constraint::Length(0),
    };

    let main_view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(85), help_constraint].as_ref())
        .split(area);

    // File explorer
    f.render_stateful_widget(
        draw_file_list(&app.file_list.current_directory, &app.file_list.items),
        main_view[0],
        &mut app.file_list.state,
    );

    // Help
    if app.state.help_visible {
        f.render_widget(draw_help_panel(), main_view[1]);
    }
}

fn draw_file_list<'a>(title_path: &'a str, files: &'a [FileEntry]) -> List<'a> {
    let items: Vec<ListItem> = files
        .iter()
        .map(|x| {
            ListItem::new(Spans::from(Span::styled(&x.name, Style::default())))
                .style(Style::default())
        })
        .collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title_path))
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

fn draw_help_panel<'a>() -> Paragraph<'a> {
    let help_text = vec![
        Spans::from("h: Toogle help"),
        Spans::from("l: Toggle logs"),
        Spans::from("f: Focus file viewer"),
        Spans::from("q: Quit"),
        Spans::from(""),
        Spans::from(Span::styled(
            "File viewer only",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Spans::from("\u{2190}: Directory up"),
        Spans::from("\u{2192}: Enter directory"),
        Spans::from("\u{2191}: Select file up"),
        Spans::from("\u{2193}: Select file down"),
    ];

    Paragraph::new(help_text.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().add_modifier(Modifier::BOLD))
                .title("Help"),
        )
        .style(Style::default().remove_modifier(Modifier::BOLD))
}

fn draw_log_view<'a>() -> TuiLoggerWidget<'a> {
    TuiLoggerWidget::default()
        .style_error(Style::default().fg(Color::Red))
        .style_debug(Style::default().fg(Color::Green))
        .style_warn(Style::default().fg(Color::Yellow))
        .style_trace(Style::default().fg(Color::Gray))
        .style_info(Style::default().fg(Color::Blue))
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default())
                .borders(Borders::ALL),
        )
        .style(Style::default())
        .output_target(false)
        .output_line(false)
        .output_file(false)
        .output_separator(' ')
}
