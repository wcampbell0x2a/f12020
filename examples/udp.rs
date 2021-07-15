use deku::DekuContainerRead;
use f12020::{Packet, PacketType};
use std::io;
use std::net::UdpSocket;
use std::time::Duration;
use tui;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Row, Table, Tabs};
use tui::Terminal;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("couldn't bind to address");
    println!("connected");

    let mut last_current_lap_best_time = 1;
    let mut recorded_lap_times = vec![];

    let stdout = io::stdout();
    let mut backend = CrosstermBackend::new(stdout);
    backend.clear();
    let mut terminal = Terminal::new(backend).unwrap();

    loop {
        let mut buf = [0; 5000];
        let (amt, addr) = socket.recv_from(&mut buf).unwrap();

        let buf = &mut buf[..amt];
        let packet = Packet::from_bytes((buf, 0)).unwrap().1;

        if let PacketType::LapData(lap_data) = packet.packet_type {
            if packet.player_car_index == 255 {
                continue;
            }
            let current_lap = &lap_data.lap_data[packet.player_car_index as usize];

            let lap_time = Duration::from_secs_f32(current_lap.current_lap_time);

            if last_current_lap_best_time != current_lap.current_lap_num {
                recorded_lap_times.push(current_lap.last_lap_time);
                last_current_lap_best_time += 1;
            }
        }
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                    .split(f.size());

                let titles = ["Lap Time(F1)"].iter().cloned().map(Spans::from).collect();
                let tab = Tabs::new(titles)
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Yellow))
                    .divider(DOT);
                f.render_widget(tab, chunks[0]);

                let mut rows = vec![];
                for (i, recorded_lap_time) in recorded_lap_times.iter().enumerate() {
                    let min = (recorded_lap_time / 60.0 as f32).floor() as u32;
                    let sec = recorded_lap_time % 60.0;
                    let sec = if sec < 10.0 {
                        format!("0{}", sec)
                    } else {
                        format!("{}", sec)
                    };
                    rows.push(Row::new(vec![
                        format!("[{}]", i + 1),
                        format!("{}:{}", min, sec),
                    ]));
                }

                let lap_time_table = Table::new(rows)
                    // You can set the style of the entire Table.
                    .style(Style::default().fg(Color::White))
                    // It has an optional header, which is simply a Row always visible at the top.
                    .header(
                        Row::new(vec!["#", "Time"])
                            .style(
                                Style::default()
                                    .fg(Color::White)
                                    .add_modifier(Modifier::BOLD),
                            )
                            // If you want some space between the header and the rest of the rows, you can always
                            // specify some margin at the bottom.
                            .bottom_margin(1),
                    )
                    .block(Block::default().title("Lap Time").borders(Borders::ALL))
                    .widths(&[Constraint::Length(5), Constraint::Length(95)])
                    .column_spacing(1);

                f.render_widget(lap_time_table, chunks[1]);
            })
            .unwrap();
    }
}
