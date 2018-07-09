use player::player_impl::PlayerImpl;

#[cfg(feature = "gst")]
use backends::gstreamer::player::GStreamerPlayer;

pub struct Player {}

impl Player {
    fn new() -> PlayerImpl {
        #[cfg(not(feature = "gst"))]
        Player {}

        #[cfg(feature = "gst")]
        GStreamerPlayer::new() as PlayerImpl
    }
}
