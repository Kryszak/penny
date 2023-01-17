use super::App;
use crate::{files::FileEntry, player::Mp3Player};
use std::vec;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};
use tui_logger::TuiLoggerWidget;

/// Render UI based on application state
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let main_view_constraint = Constraint::Max(80);
    let now_playing_view_constraint = Constraint::Percentage(20);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([main_view_constraint, now_playing_view_constraint].as_ref())
        .split(f.size());

    let (main_view_area, player_area) = (chunks[0], chunks[1]);

    render_main_view(f, main_view_area, app);

    // Player
    draw_player_panel(f, &mut app.player, player_area);
}

fn render_main_view<B: Backend>(f: &mut Frame<B>, area: Rect, app: &mut App) {
    let help_constraint = match app.state.help_visible {
        true => Constraint::Percentage(15),
        false => Constraint::Length(0),
    };

    let logs_constraint = match app.state.logs_visible {
        true => Constraint::Percentage(25),
        false => Constraint::Length(0),
    };

    let main_view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(60), help_constraint, logs_constraint].as_ref())
        .split(area);

    let (file_viewer_area, help_area, logs_area) = (main_view[0], main_view[1], main_view[2]);

    // File explorer
    f.render_stateful_widget(
        draw_file_list(&app.file_list.current_directory, &app.file_list.items),
        file_viewer_area,
        &mut app.file_list.state,
    );

    // Help
    f.render_widget(draw_help_panel(app.state.file_viewer_focused), help_area);

    // Logs
    f.render_widget(draw_log_view(), logs_area);
}

fn draw_file_list<'a>(title_path: &'a str, files: &'a [FileEntry]) -> List<'a> {
    let items: Vec<ListItem> = files
        .iter()
        .map(|x| {
            ListItem::new(Spans::from(Span::styled(&x.name, Style::default())))
                .style(Style::default().remove_modifier(Modifier::BOLD))
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title_path)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("> ")
}

fn draw_player_panel<B: Backend>(f: &mut Frame<B>, player: &mut Mp3Player, area: Rect) {
    let view = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(58),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);

    let (song_info_area, progress_bar_area) = (view[0], view[2]);

    let block_title = player.get_playback_status_string();

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(block_title)
            .style(Style::default().add_modifier(Modifier::BOLD)),
        area,
    );

    // Song info
    f.render_widget(draw_song_info(player), song_info_area);
    // Song progress bar
    f.render_widget(draw_song_progress(player), progress_bar_area);
}

fn draw_song_info<'a>(player: &mut Mp3Player) -> Paragraph<'a> {
    let lines: Vec<Spans> = player
        .display_information()
        .into_iter()
        .map(Spans::from)
        .collect();
    Paragraph::new(lines)
        .block(Block::default())
        .style(Style::default().remove_modifier(Modifier::BOLD))
}

fn draw_song_progress<'a>(player: &Mp3Player) -> Gauge<'a> {
    let label = player
        .get_text_progress()
        .unwrap_or_else(|| String::from("-/-"));
    Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(Color::Cyan))
        .ratio(player.get_current_song_percentage_progress())
        .label(Span::styled(
            label,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
}

fn draw_help_panel<'a>(show_file_viewer_help: bool) -> Paragraph<'a> {
    let mut help_text = vec![
        Spans::from("h: Toogle help"),
        Spans::from("f: Focus file viewer"),
        Spans::from("\u{23CE}: Play selected file"),
        Spans::from("q: Quit"),
    ];

    let mut player_help = vec![
        Spans::from(""),
        Spans::from(Span::styled(
            "Player",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Spans::from("p: Play/Pause"),
        Spans::from("s: Stop"),
    ];

    help_text.append(&mut player_help);

    if show_file_viewer_help {
        let mut file_viewer_help_text = vec![
            Spans::from(""),
            Spans::from(Span::styled(
                "File viewer",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Spans::from("\u{2190}: Directory up"),
            Spans::from("\u{2192}: Enter directory"),
            Spans::from("\u{2191}: Select file up"),
            Spans::from("\u{2193}: Select file down"),
        ];
        help_text.append(&mut file_viewer_help_text);
    }
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
        .style_error(
            Style::default()
                .fg(Color::Red)
                .remove_modifier(Modifier::BOLD),
        )
        .style_debug(
            Style::default()
                .fg(Color::Green)
                .remove_modifier(Modifier::BOLD),
        )
        .style_warn(
            Style::default()
                .fg(Color::Yellow)
                .remove_modifier(Modifier::BOLD),
        )
        .style_trace(
            Style::default()
                .fg(Color::Gray)
                .remove_modifier(Modifier::BOLD),
        )
        .style_info(
            Style::default()
                .fg(Color::Blue)
                .remove_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .title("Logs")
                .border_style(Style::default())
                .borders(Borders::ALL)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .output_level(None)
        .style(Style::default())
        .output_target(false)
        .output_line(false)
        .output_file(false)
        .output_timestamp(None)
        .output_separator(' ')
}
