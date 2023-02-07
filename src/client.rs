use crate::client::app::AppState;
use crate::client::events::{select_event, ticker, AppEvent};
use crate::morser::messenger_client::MessengerClient;
use crate::morser::Signal;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, ExecutableCommand};
use rdev::{grab, listen, Event as REvent, EventType, Key};
use rodio::source::SineWave;
use rodio::{OutputDevices, OutputStream, Sink};
use std::fmt::{Display, Write};
use std::sync::mpsc as smpsc;
use std::time::Duration;
use std::{process, thread};
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncRead};
use tokio::join;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use tokio_stream::{wrappers, StreamExt};
use tonic::Streaming;
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::CompletedFrame;
use tui::Terminal;

mod app;
mod events;
mod morse;

async fn singer(mut stream: Streaming<Signal>, sink: Sink) {
    while let Some(result) = stream.next().await {
        let value = result.expect("Couldn't read from provided stream");
        if value.state {
            sink.play();
        } else {
            sink.pause();
        }
    }

    ()
}

fn event_listener(tx: UnboundedSender<bool>) {
    let callback = move |event: REvent| -> Option<REvent> {
        match event.event_type {
            EventType::KeyPress(Key::Space) => {
                tx.send(true).unwrap();
            }
            EventType::KeyRelease(Key::Space) => {
                tx.send(false).unwrap();
            }
            _ => {}
        }

        Some(event)
    };

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

async fn change_manager(mut rx: UnboundedReceiver<bool>, tx: Sender<Signal>) {
    let mut state = false;

    while let Some(value) = rx.recv().await {
        if value != state {
            state = !state;
            tx.send(Signal { state }).await.unwrap();
        }
    }
}

pub async fn execute1() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessengerClient::connect("http://192.168.1.41:50051")
        .await
        .expect("Couldn't connect");

    let (to_server, rx) = mpsc::channel(8);

    let out_stream = wrappers::ReceiverStream::new(rx);
    let response = client.chat(out_stream).await.expect("couldn't chat");

    let in_stream = response.into_inner();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.pause();
    let source = SineWave::new(440.0);
    sink.append(source);

    let (tx, rx) = mpsc::unbounded_channel();

    let change = tokio::spawn(change_manager(rx, to_server));

    let singer = tokio::spawn(singer(in_stream, sink));
    let event_listener = thread::spawn(move || event_listener(tx));

    change.await?;
    singer.await?;
    event_listener.join().unwrap();

    Ok(())
}

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    setup_terminal()?;

    let mut terminal = start_terminal(std::io::stdout())?;

    run_app(&mut terminal).await?;

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = AppState::new(200);

    let mut reader = EventStream::new();
    let (tx_r, mut rx_r) = mpsc::unbounded_channel();
    let (tx_t, mut rx_t) = mpsc::unbounded_channel();

    tokio::task::spawn_blocking(|| system_signal(tx_r));
    tokio::spawn(ticker(app.tick_rate_d(), tx_t));

    loop {
        let event = select_event(&mut rx_t, &mut reader, &mut rx_r).await;

        match event {
            AppEvent::Tick => {
                app.on_tick();
            }
            AppEvent::CEvent(event) => app.handle_c_event(event),
            AppEvent::SysSigOff => app.set_signal(false),
            AppEvent::SysSigOn => app.set_signal(true),
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
    eprintln!("Started system sinal");
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
