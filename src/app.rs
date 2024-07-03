use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    symbols::border,
    widgets::{
        block::{Position, Title}, Block, List, ListState, Paragraph, Widget
    },
};

use num_traits::FromPrimitive;

use ratatui::prelude::*;

use crate::{components::ship_status::ShipStatus, tui};

const TITLE_HEADER: &str = r#"
     _                      _______                      _      
  _dMMMb._              .adOOOOOOOOOba.              _,dMMMb_   
 dP'  ~YMMb            dOOOOOOOOOOOOOOOb            aMMP~  `Yb  
 V      ~"Mb          dOOOOOOOOOOOOOOOOOb          dM"~      V  
          `Mb.       dOOOOOOOOOOOOOOOOOOOb       ,dM'           
           `YMb._   |OOOOOOOOOOOOOOOOOOOOO|   _,dMP'            
      __     `YMMM| OP'~"YOOOOOOOOOOOP"~`YO |MMMP'     __       
    ,dMMMb.     ~~' OO     `YOOOOOP'     OO `~~     ,dMMMb.     
 _,dP~  `YMba_      OOb      `OOO'      dOO      _aMMP'  ~Yb._  
             `YMMMM\`OOOo     OOO     oOOO'/MMMMP'              
     ,aa.     `~YMMb `OOOb._,dOOOb._,dOOO'dMMP~'       ,aa.     
   ,dMYYMba._         `OOOOOOOOOOOOOOOOO'          _,adMYYMb.   
  ,MP'   `YMMba._      OOOOOOOOOOOOOOOOO       _,adMMP'   `YM.  
  MP'        ~YMMMba._ YOOOOPVVVVVYOOOOP  _,adMMMMP~       `YM  
  YMb           ~YMMMM\`OOOOI`````IOOOOO'/MMMMP~           dMP  
   `Mb.           `YMMMb`OOOI,,,,,IOOOO'dMMMP'           ,dM'   
     `'                  `OObNNNNNdOO'                   `'     
                           `~OOOOO~'                            

M0TH3R@3-OS
"#;

#[derive(Debug, Copy, Clone, num_derive::FromPrimitive, strum::AsRefStr)]
enum MenuItem {
    Map = 0,
    Crew,
    Info,
}

#[derive(Debug)]
pub struct App {
    selected: MenuItem,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected: MenuItem::Map,
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                self.handle_press_event(key);
            }
        }
        Ok(())
    }

    fn handle_press_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q')  => { self.exit = true; },
            KeyCode::Up         => { self.select(false); },
            KeyCode::Down       => { self.select(true); },
            KeyCode::Enter      => {},
            _ => {},
        }
    }

    fn select(&mut self, below: bool) {
        let value = if below { -1 } else { 1 };
        self.selected = match FromPrimitive::from_i8(self.selected as i8 + value) {
            Some(d2) => d2,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }

    fn render_title(&mut self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
                " Select ".into(),
                "<Enter>".green().bold(),
                " Move up ".into(),
                "<Up>".green().bold(),
                " Move down ".into(),
                "<Down>".green().bold(),
                " Quit ".into(),
                "<Q> ".green().bold(),
        ]));
        let block = Block::bordered()
            .title(
                instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let text = Text::from(TITLE_HEADER)
            .fg(Color::Green);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let mut list_state = ListState::default();
        let menu = List::new([MenuItem::Map.as_ref(), MenuItem::Crew.as_ref(), MenuItem::Info.as_ref()])
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);
        
        ratatui::prelude::StatefulWidget::render(menu, area, buf, &mut list_state);
    }
}


impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, right] = Layout::horizontal([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ]).areas(area);

        let [title, list, ship_status] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(25),
                Constraint::Percentage(40),
            ])
            .areas(left);

        self.render_title(title, buf);
        self.render_list(list, buf);
        ShipStatus.render(ship_status, buf);

        let title = Title::from(" Map ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);
        block.render(right, buf);
    }
}


