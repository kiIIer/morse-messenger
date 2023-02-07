use rodio::source::SineWave;
use rodio::{OutputStream, OutputStreamHandle, Sink};
use tokio::sync::mpsc::UnboundedReceiver;

pub fn setup_sink() -> (OutputStream, OutputStreamHandle, Sink) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.pause();
    let source = SineWave::new(440.0);
    sink.append(source);
    (stream, stream_handle, sink)
}

pub async fn singer(mut rx: UnboundedReceiver<bool>, sink: Sink) {
    while let Some(value) = rx.recv().await {
        if value {
            sink.play();
        } else {
            sink.pause();
        }
    }
}
