use std::time::{Duration, Instant};
use ratatui::crossterm::event::{self, Event, KeyEvent, KeyEventKind, KeyCode};
use ratatui::widgets::{Paragraph, Block, BorderType, Borders};
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Color, Stylize};
use ratatui::text::{Line, Span};


const TICK_RATE: Duration = Duration::from_millis(500);
const Muted: Color = Color::DarkGray;
const ACCENT: Color = Color::Cyan;

struct App{
    running: bool,
}

fn main()-> std::io::Result<()> {
    ratatui::run(| terminal | App::new().run(terminal))
}
impl App {
    fn new()-> Self {
        Self {
            running: true,
        }
    }

     fn run(&mut self, terminal: &mut DefaultTerminal)-> std::io::Result<()> {
         let mut last_tick = Instant::now();

         while self.running {

             (terminal).draw(|frame| self.render(frame))?;

             let timeout = TICK_RATE.saturating_sub(last_tick.elapsed());

             if event::poll(timeout)? {
                 if let Event::Key(key) = event::read()? {
                     self.handle_key(key);
                 }
             }

             if last_tick.elapsed() >= TICK_RATE {
                 self.on_tick();
                 last_tick = Instant::now();
             }
         }

         Ok(())
     }

     fn handle_key(&mut self, key: KeyEvent) {
         if key.kind != KeyEventKind::Press {
             return;
         }
         match key.code {
             KeyCode::Char('q') | KeyCode::Esc => self.running = false,
             _=> {}
         }
     }

    fn on_tick(&mut self) {

    }

    fn render(&mut self, frame: &mut Frame) {
       let [header, top, history, processes, footer] = Layout::vertical([
           Constraint::Length(3),
           Constraint::Length(10),
           Constraint::Length(10),
           Constraint::Fill(1),
           Constraint::Length(1),
       ]).areas(frame.area());

        let [cpu, memory] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).areas(top);

        self.render_header(frame, header);
        (frame).render_widget(panel("CPU"), cpu);
        (frame).render_widget(panel("Memory"), memory);
        (frame).render_widget(panel("History"), history);
        (frame).render_widget(panel("Processes"), processes);
        self.render_footer(frame, footer);

    }

    fn render_header(&mut self, frame: &mut Frame, area: Rect) {
       let sep = Span::styled(" | ", Muted);
        let line = Line::from(vec![
            Span::styled("KillMonit", Style::default().bold().fg(ACCENT)),
            sep,
            Span::raw("Live system monitor")
        ]);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Muted);

        (frame).render_widget(Paragraph::new(line).block(block), area);
    }

    fn render_footer(&mut self, frame: &mut Frame, area: Rect) {
        let key = |k: &'static str, desc: &'static str| {
            [
               Span::styled(format!(" {k} "), Style::new().fg(Color::Black).bg(ACCENT)),
                Span::styled(format!(" {desc}\t "), Muted),
            ]
        };

        let spans: Vec<Span> =[
            key("q", "Quit")
        ]
            .into_iter()
            .flatten()
            .collect();

        (frame).render_widget(Line::from(spans), area)
    }

}

fn panel(title: &str) -> Block<'static> {
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Muted)
        .title(Line::from(format!("{title}")).fg(ACCENT).bold())
}