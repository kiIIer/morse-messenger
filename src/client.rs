use crate::client::app::AppState;
use crate::client::events::{select_event, ticker, AppEvent};
use crate::client::morse::{letter_receiver, letter_transmitter};
use crate::client::sound::{setup_sink, singer};
use crate::morser::messenger_client::MessengerClient;
use crate::morser::Signal;
use crossterm::event::EventStream;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use rdev::{listen, Event as REvent, EventType, Key};
use std::{env, process};
use tokio::io;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::{wrappers, StreamExt};
use tonic::transport::Uri;
use tonic::Streaming;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

mod app;
mod events;
mod morse;
mod sound;

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        args = vec![
            "smth".to_string(),
            "50".to_string(),
            "100".to_string(),
            "0.25".to_string(),
            "http://192.168.1.41:50051".to_string(),
        ]
    }

    let tick_rate: usize = args[1].parse().expect("Couldn't parse arg");
    let time_unit: usize = args[2].parse().expect("Couldn't parse arg");
    let precision: f64 = args[3].parse().expect("Couldn't parse arg");
    let server_address = &args[4];

    let mut client =
        MessengerClient::connect(server_address.parse::<Uri>().expect("That's not uri"))
            .await
            .expect("Couldn't connect");
    let (tx_server, rx) = mpsc::unbounded_channel();

    let out_stream = wrappers::UnboundedReceiverStream::new(rx);

    let response = client.chat(out_stream).await.expect("couldn't chat");
    let rx_server = response.into_inner();

    chain_hook();
    setup_terminal()?;

    let mut terminal = start_terminal(std::io::stdout())?;

    run_app(
        &mut terminal,
        tx_server,
        rx_server,
        tick_rate,
        time_unit,
        precision,
    )
    .await?;

    Ok(())
}

async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    to_server: UnboundedSender<Signal>,
    mut from_server: Streaming<Signal>,
    tick_rate: usize,
    time_unit: usize,
    precision: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = EventStream::new();
    let (tx_r, mut rx_r) = mpsc::unbounded_channel();
    let (tx_t, mut rx_t) = mpsc::unbounded_channel();
    let (tx_s, rx_s) = mpsc::unbounded_channel();
    let (tx_l, rx_l) = mpsc::unbounded_channel();
    let (tx_c, mut rx_c) = mpsc::unbounded_channel();
    let (tx_sl, mut rx_sl) = mpsc::unbounded_channel();
    let (tx_server1, rx_server1) = mpsc::unbounded_channel();
    let (tx_server2, rx_server2) = mpsc::unbounded_channel();

    let (_stream, _stream_handle, sink) = setup_sink();

    let mut app = AppState::new(
        tick_rate as i32,
        time_unit as i32,
        precision,
        tx_s,
        to_server.clone(),
        tx_l,
        rx_server1,
    );

    tokio::task::spawn_blocking(|| system_signal(tx_r));
    tokio::spawn(ticker(app.tick_rate_d(), tx_t));
    tokio::spawn(singer(rx_s, sink));
    tokio::spawn(letter_transmitter(rx_l, to_server, tx_c, app.time_unit_d()));
    tokio::spawn(async move {
        while let Some(Ok(signal)) = from_server.next().await {
            tx_server1
                .send(signal.clone())
                .expect("Couldn't duplicate signal");
            tx_server2
                .send(signal.clone())
                .expect("Couldn't duplicate signal");
        }
    });
    tokio::spawn(letter_receiver(
        rx_server2,
        tx_sl,
        app.time_unit_d(),
        app.precision(),
    ));

    loop {
        let event = select_event(
            &mut rx_t,
            &mut reader,
            &mut rx_r,
            app.rx_server(),
            &mut rx_c,
            &mut rx_sl,
        )
        .await;

        match event {
            AppEvent::Tick => {
                app.on_tick();
            }
            AppEvent::CEvent(event) => app.handle_c_event(event),
            AppEvent::SysSigOff => app.signal_off(),
            AppEvent::SysSigOn => app.signal_on(),
            AppEvent::Server(signal) => app.set_signal(signal.state),
            AppEvent::CountWord => app.count_word(),
            AppEvent::AddLetter(letter) => app.add_letter(letter),
        }

        if app.should_quit() {
            shutdown_terminal();
            process::exit(0);
        }

        draw(terminal, &app)?;
    }
}

fn setup_terminal() -> Result<(), io::Error> {
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    Ok(())
}

fn start_terminal<W: std::io::Write>(buf: W) -> io::Result<Terminal<CrosstermBackend<W>>> {
    let backend = CrosstermBackend::new(buf);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(terminal)
}

fn shutdown_terminal() {
    std::io::stdout()
        .execute(LeaveAlternateScreen)
        .expect("Couldn't exit Alternate Screen");
    disable_raw_mode().expect("Couldn't disable raw mode");
}

fn draw<'a, B: Backend>(terminal: &mut Terminal<B>, app: &AppState) -> io::Result<()> {
    terminal.draw(|f| {
        app.draw(f);
    })?;

    Ok(())
}

fn system_signal(tx: UnboundedSender<AppEvent>) {
    eprintln!("Started system signal");
    let mut inner_state = Box::new(false);
    let callback = move |event: REvent| match event.event_type {
        EventType::KeyPress(Key::Space) => {
            if !*inner_state {
                *inner_state = !*inner_state;
                tx.send(AppEvent::SysSigOn).unwrap();
            }
        }
        EventType::KeyRelease(Key::Space) => {
            if *inner_state {
                *inner_state = !*inner_state;
                tx.send(AppEvent::SysSigOff).unwrap();
            }
        }
        _ => {}
    };
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        shutdown_terminal();
        original_hook(panic);
    }));
}
