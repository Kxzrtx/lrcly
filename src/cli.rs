use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lrcly")]
#[command(about = "Live synchronized lyrics for your currently playing song")]
pub struct Args {
    ///only_current_lyric
    #[arg(long, conflicts_with_all = ["info", "time", "full"])]
    pub lyrics: bool,

    ///info_clyr
    #[arg(long, conflicts_with_all = ["lyrics", "time", "full"])]
    pub info: bool,

    ///clyr_playback
    #[arg(long, conflicts_with_all = ["lyrics", "info", "full"])]
    pub time: bool,

    ///full
    #[arg(long, conflicts_with_all = ["lyrics", "info", "time"])]
    pub full: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum DisplayMode {
    Lyrics,
    Info,
    Time,
    Full,
}
