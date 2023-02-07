use rodio::source::SineWave;
use rodio::{OutputStream, Sink};
use tokio::sync::mpsc::UnboundedReceiver;

fn setup_sink() -> Sink {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.pause();
    let source = SineWave::new(440.0);
    sink.append(source);

    sink
}

pub async fn singer(mut rx: UnboundedReceiver<bool>) {
    let sink = setup_sink();
    while let Some(value) = rx.recv().await {
        if value {
            sink.play();
        } else {
            sink.pause();
        }
    }

    ()
}
