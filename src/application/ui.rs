use super::{app::VisualizationStyle, App};
use crate::logger::logger_widget::TuiLoggerWidget;
use crate::queue::SongFile;
use crate::{files::FileEntry, player::Mp3Player};
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, BarChart, BorderType, Borders, Chart, Dataset, Gauge, GraphType, List, ListItem,
        Paragraph,
    },
    Frame,
};
use std::vec;

/// Render UI based on application state
pub fn ui(f: &mut Frame, app: &mut App) {
    let main_view_constraint = Constraint::Max(60);
    let now_playing_view_constraint = Constraint::Percentage(40);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([main_view_constraint, now_playing_view_constraint].as_ref())
        .split(f.size());

    let (main_view_area, player_area) = (chunks[0], chunks[1]);

    render_main_view(f, main_view_area, app);

    // Player
    draw_player_panel(f, app, player_area);
}

fn render_main_view(f: &mut Frame, area: Rect, app: &mut App) {
    let help_constraint = match app.state.help_visible {
        true => Constraint::Max(35),
        false => Constraint::Length(0),
    };

    let logs_constraint = match app.state.logs_visible {
        true => Constraint::Max(25),
        false => Constraint::Length(0),
    };

    let main_view = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Max(60),
                Constraint::Min(30),
                help_constraint,
                logs_constraint,
            ]
            .as_ref(),
        )
        .split(area);

    let (file_viewer_area, queue_view_area, help_area, logs_area) =
        (main_view[0], main_view[1], main_view[2], main_view[3]);

    // File explorer
    f.render_stateful_widget(
        draw_file_list(
            &app.file_list.current_directory,
            &app.file_list.items,
            app.state.color_style,
            app.state.file_viewer_focused,
        ),
        file_viewer_area,
        &mut app.file_list.state,
    );

    // Playing queue
    f.render_stateful_widget(
        draw_queue_list(
            "Queue",
            &app.queue_view.items,
            app.queue_view.now_playing,
            app.state.color_style,
            !app.state.file_viewer_focused,
        ),
        queue_view_area,
        &mut app.queue_view.state,
    );

    // Help
    f.render_widget(draw_help_panel(app.state.file_viewer_focused), help_area);

    // Logs
    f.render_widget(draw_log_view(), logs_area);
}

fn draw_file_list<'a>(
    title_path: &'a str,
    files: &'a [FileEntry],
    color: Color,
    focused: bool,
) -> List<'a> {
    let items: Vec<ListItem> = files
        .iter()
        .map(|x| {
            ListItem::new(Line::from(Span::styled(&x.name, Style::default())))
                .style(Style::default().remove_modifier(Modifier::BOLD))
        })
        .collect();

    let (border_type, border_color) = get_border_style(focused, color);

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(Style::default().fg(border_color))
                .title(title_path)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().bg(color).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}

fn draw_queue_list<'a>(
    title_path: &'a str,
    items: &'a [SongFile],
    now_playing: Option<usize>,
    color: Color,
    focused: bool,
) -> List<'a> {
    let items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(index, x)| {
            if let Some(i) = now_playing {
                if i == index {
                    return ListItem::new(Line::from(Span::styled(
                        format!("\u{25B6} {}", x.display_short()),
                        Style::default(),
                    )));
                }
            }
            ListItem::new(Line::from(Span::styled(
                format!("  {}", x.display_short()),
                Style::default(),
            )))
            .style(Style::default().remove_modifier(Modifier::BOLD))
        })
        .collect();

    let (border_type, border_color) = get_border_style(focused, color);

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
                .border_style(Style::default().fg(border_color))
                .title(title_path)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .highlight_style(Style::default().bg(color).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ")
}

fn get_border_style(focused: bool, accent_color: Color) -> (BorderType, Color) {
    match focused {
        true => (BorderType::Double, accent_color),
        false => (BorderType::Plain, Color::White),
    }
}

fn draw_player_panel(f: &mut Frame, app: &mut App, area: Rect) {
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

    let (song_info_area, audio_spectrum_area, progress_bar_area) = (view[0], view[1], view[2]);

    let block_title = app.player.get_playback_status_string();

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(block_title)
            .style(Style::default().add_modifier(Modifier::BOLD)),
        area,
    );

    // Song info
    f.render_widget(draw_song_info(&mut app.player), song_info_area);

    // audio spectrum
    draw_audio_spectrum(f, app, audio_spectrum_area);

    // Song progress bar
    f.render_widget(
        draw_song_progress(&app.player, app.state.color_style),
        progress_bar_area,
    );
}

fn draw_song_info(player: &mut Mp3Player) -> Paragraph {
    let lines: Vec<Line> = player
        .display_information()
        .into_iter()
        .map(Line::from)
        .collect();
    Paragraph::new(lines)
        .block(Block::default())
        .style(Style::default().remove_modifier(Modifier::BOLD))
}

fn draw_song_progress(player: &Mp3Player, color: Color) -> Gauge {
    let label = player
        .get_text_progress()
        .unwrap_or_else(|| String::from("-/-"));
    Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(color))
        .ratio(player.get_current_song_percentage_progress())
        .label(Span::styled(
            label,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
}

fn draw_audio_spectrum(f: &mut Frame, app: &mut App, rect: Rect) {
    match app.state.visualization_style {
        VisualizationStyle::Bar { ref mut data } => {
            let unsigned_spectrum: Vec<u64> = app
                .player
                .get_audio_spectrum()
                .into_iter()
                .map(|v| v as u64)
                .collect();
            data.update_spectrum(unsigned_spectrum);
            f.render_widget(
                BarChart::default()
                    .data(&data.audio_spectrum)
                    .bar_width(rect.width / data.audio_spectrum_band_count as u16)
                    .style(Style::default().fg(app.state.color_style))
                    .value_style(Style::default().fg(app.state.color_style)),
                rect,
            );
        }
        VisualizationStyle::Chart { ref mut data } => {
            let unsigned_spectrum: Vec<f64> = app
                .player
                .get_audio_spectrum()
                .into_iter()
                .map(f64::from)
                .collect();
            data.update_spectrum(unsigned_spectrum);
            let dataset = Dataset::default()
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(app.state.color_style))
                .graph_type(GraphType::Line)
                .data(&data.audio_spectrum);
            f.render_widget(
                Chart::new(vec![dataset])
                    .block(Block::default())
                    .x_axis(Axis::default().bounds([0.0, data.audio_spectrum_point_count as f64]))
                    .y_axis(Axis::default().bounds([0.0, data.max_value])),
                rect,
            );
        }
    }
}

fn draw_help_panel<'a>(show_file_viewer_help: bool) -> Paragraph<'a> {
    let mut help_text = vec![
        Line::from("h: Toogle help"),
        Line::from("f: Focus files/queue"),
        Line::from("v: Change visualization style"),
        Line::from("c: Change player color"),
        Line::from("q: Quit"),
    ];

    let mut player_help = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Player",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("p: Toggle playback"),
        Line::from("s: Stop"),
        Line::from("j: Play previous"),
        Line::from("k: Play next"),
    ];

    help_text.append(&mut player_help);

    if !show_file_viewer_help {
        let mut queue_view_help_test = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Playback queue",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("\u{23CE}: Play song"),
            Line::from("\u{2191}: Select song up"),
            Line::from("\u{2193}: Select song down"),
            Line::from("d: Remove song"),
        ];
        help_text.append(&mut queue_view_help_test);
    }

    if show_file_viewer_help {
        let mut file_viewer_help_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "File viewer",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from("\u{23CE}: Add to queue"),
            Line::from("\u{2190}: Directory up"),
            Line::from("\u{2192}: Enter directory"),
            Line::from("\u{2191}: Select file up"),
            Line::from("\u{2193}: Select file down"),
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
        .style(Style::default())
        .output_separator(' ')
}
