use player::metadata::Metadata;
use player::frame::FrameRenderer;

use ipc_channel::ipc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlaybackState {
    Stopped,
    // Buffering,
    Paused,
    Playing,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PlayerEvent {
    EndOfStream,
    MetadataUpdated(Metadata),
    StateChanged(PlaybackState),
    FrameUpdated,
    Error,
}

pub trait PlayerImpl {
    fn register_event_handler(&self, sender: ipc::IpcSender<PlayerEvent>);
    fn register_frame_renderer<R: FrameRenderer>(&self, renderer: R);

    fn setup(&self) -> bool;
    fn play(&self);
    fn stop(&self);

    fn set_input_size(&self, size: u64);
    fn push_data(&self, data: Vec<u8>) -> bool;
    fn end_of_stream(&self) -> bool;
}
