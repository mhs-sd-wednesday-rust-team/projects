use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, Widget},
};
use tui_big_text::{BigText, PixelSize};

pub struct FinishMenuView;

impl Widget for FinishMenuView {
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
            .lines(vec!["Game".red().into(), "Finished".red().into()])
            .build()
            .render(layout[1], buf);

        Paragraph::new("Press Enter to restart. Press 'q' to quit.")
            .centered()
            .render(layout[2], buf);
    }
}
