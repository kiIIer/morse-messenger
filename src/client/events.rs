use crossterm::event::{Event as CEvent, EventStream};
use futures::FutureExt;
use futures_timer::Delay;
use std::time::Duration;
use tokio::select;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub enum AppEvent {
    Tick,
    CEvent(CEvent),
}

pub async fn select_event(tick_rate: Duration, reader: &mut EventStream) -> AppEvent {
    let delay = Delay::new(tick_rate).fuse();
    select! {
        _ = delay => {return AppEvent::Tick},
        Some(Ok(c_event)) = reader.next().fuse() => {return AppEvent::CEvent(c_event)}
    }
}
