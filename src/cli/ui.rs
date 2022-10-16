use crate::cli::cli_model::{App, WidgetMode};
use tui::{
   backend::{Backend},
   layout::{Constraint, Direction, Layout},
   style::{Color, Modifier, Style},
   text::{Span, Spans, Text},
   widgets::{Block, Borders, Cell, Paragraph, Row, Table},
   Frame
};
use unicode_width::UnicodeWidthStr;


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
   let chunks = Layout::default()
       .direction(Direction::Vertical)
       .margin(2)
       .constraints(
           [
               Constraint::Length(1),
               Constraint::Length(3),
               Constraint::Min(1),
           ]
           .as_ref(),
       )
       .split(f.size());

   //---------------------------INPUT------------------------
   let (msg, style) = match app.view {
       WidgetMode::SearchResults => (
           vec![
               Span::raw("Press "),
               Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
               Span::raw(" to exit, "),
               Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
               Span::raw(" to start new search."),
           ],
           Style::default().add_modifier(Modifier::RAPID_BLINK),
       ),
       WidgetMode::Search => (
           vec![
               Span::raw("Press "),
               Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
               Span::raw(" to switch to results."),
           ],
           Style::default(),
       ),
   };
   let mut text = Text::from(Spans::from(msg));
   text.patch_style(style);
   let help_message = Paragraph::new(text);
   f.render_widget(help_message, chunks[0]);

   let input = Paragraph::new(app.search.as_ref())
       .style(match app.view {
           WidgetMode::SearchResults => Style::default(),
           WidgetMode::Search => Style::default().fg(Color::LightYellow),
       })
       .block(Block::default().borders(Borders::ALL).title("Input"));
   f.render_widget(input, chunks[1]);

   match app.view {
       WidgetMode::SearchResults =>
           // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
           {}

       WidgetMode::Search => {
           // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
           f.set_cursor(
               // Put cursor past the end of the input text
               chunks[1].x + app.search.width() as u16 + 1,
               // Move one line down, from the border to the input line
               chunks[1].y + 1,
           )
       }
   }
   //---------------------------INPUT------------------------
   let selected_style = Style::default().add_modifier(Modifier::REVERSED);
   let normal_style = Style::default().bg(Color::Blue);
   let header_cells = ["Path", "line", "char"]
       .iter()
       .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
   let header = Row::new(header_cells)
       .style(normal_style)
       .height(1)
       .bottom_margin(1);
   let rows = app.items.iter().map(|item| {
       let height = item
           .iter()
           .map(|content| content.chars().filter(|c| *c == '\n').count())
           .max()
           .unwrap_or(0)
           + 1;
       let cells = item.iter().map(|c| Cell::from(*c));
       Row::new(cells).height(height as u16).bottom_margin(1)
   });
   let t = Table::new(rows)
       .header(header)
       .block(
           Block::default()
               .borders(Borders::ALL)
               .title("Search results"),
       )
       .highlight_style(selected_style.fg(Color::LightYellow))
       .highlight_symbol(">> ")
       .widths(&[
           Constraint::Percentage(80),
           Constraint::Length(10),
           Constraint::Min(5),
       ]);
   f.render_stateful_widget(t, chunks[2], &mut app.state);
}