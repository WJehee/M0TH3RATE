use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    symbols::border,
    widgets::{
        block::{Position, Title}, Block, List, ListState, Paragraph, Widget
    },
};

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

####################################################################
"#;

#[derive(Debug)]
enum Selected {
    Status,
}

#[derive(Debug)]
pub struct App {
    selected_tab: usize,
    selected: Selected,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
            selected: Selected::Status,
            exit: false,
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(frame.size());

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(chunks[0]);

        frame.render_widget(self, left[0]);
        ShipStatus.draw(frame, left[1]);

        let title = Title::from(" Ship Status ".bold());
        let block = Block::bordered()
            .title(
                title
                    .alignment(Alignment::Center)
                    .position(Position::Bottom)
            )
            .border_set(border::THICK);

        frame.render_widget(block, left[1]);

        let title = Title::from(" Map ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        frame.render_widget(block, chunks[1]);
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
            KeyCode::Up         => { self.selected_tab = (self.selected_tab + 4 + 1) % 4; },
            KeyCode::Down       => { self.selected_tab = (self.selected_tab + 4 - 1) % 4 },
            KeyCode::Enter      => {},
            _ => {},
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
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

        let items = ["Item 1", "Item 2", "Item 3"];
        let mut list_state = ListState::default();

        ratatui::prelude::StatefulWidget::render(
            List::new(items)
            .block(Block::bordered().title("List"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
            area, buf, &mut list_state
        );
    }
}

