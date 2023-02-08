use crate::client::events::AppEvent::Tick;
use crate::morser::Signal;
use crossterm::event::{Event as CEvent, EventStream};
use futures::FutureExt;
use futures_timer::Delay;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;
use tonic::Streaming;

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    CEvent(CEvent),
    SysSigOn,
    SysSigOff,
    Server(Signal),
    CountWord,
}

pub async fn select_event(
    rx_t: &mut UnboundedReceiver<AppEvent>,
    reader: &mut EventStream,
    rx_r: &mut UnboundedReceiver<AppEvent>,
    rx_s: &mut Streaming<Signal>,
    rx_c: &mut UnboundedReceiver<()>,
) -> AppEvent {
    if let Ok(_) = rx_t.try_recv() {
        return Tick;
    }
    select! {
        Some(event) = rx_t.recv() => {
            return event;
        }
        Some(Ok(c_event)) = reader.next().fuse() => {
            return AppEvent::CEvent(c_event);
        }
        Some(event) = rx_r.recv() => {
            return event;
        }
        // TODO: Handle err
        Some(Ok(signal)) = rx_s.next() => {
            return AppEvent::Server(signal);
        }
        Some(_) = rx_c.recv() => {
            return AppEvent::CountWord
        }
    }
}

pub async fn ticker(tick_rate: Duration, tx: UnboundedSender<AppEvent>) {
    loop {
        let delay = Delay::new(tick_rate).fuse();

        delay.await;

        tx.send(Tick).expect("Couldn't send tick");
    }
}
