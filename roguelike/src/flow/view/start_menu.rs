use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

pub struct StartMenuView;

impl Widget for StartMenuView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .border_type(ratatui::widgets::BorderType::Thick)
            .render(area, buf);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Percentage(40),
                Constraint::Percentage(10),
                Constraint::Fill(1),
            ])
            .split(area);

        BigText::builder()
            .pixel_size(PixelSize::Full)
            .centered()
            .style(Style::new().blue())
            .lines(vec!["Crab".red().into(), "Knight".blue().into()])
            .build()
            .render(layout[1], buf);

        Paragraph::new("Press any character to start")
            .centered()
            .render(layout[2], buf);
    }
}
