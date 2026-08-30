mod cli;

use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use mpris::PlayerFinder;
use serde::Deserialize;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LyricsResponse {
    plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}

//lyricsdata

fn parse_lyrics(lyrics: &str) -> Vec<(f64, String)> {
    let mut lines = Vec::new();

    for line in lyrics.lines() {
        let Some(end) = line.find(']') else {
            continue;
        };

        let timestamp = &line[1..end];
        let text = line[end + 1..].trim();

        let mut parts = timestamp.split(':');

        let minutes: f64 = match parts.next().and_then(|x| x.parse().ok()) {
            Some(value) => value,
            None => continue,
        };

        let seconds: f64 = match parts.next().and_then(|x| x.parse().ok()) {
            Some(value) => value,
            None => continue,
        };

        let total_seconds = minutes * 60.0 + seconds;

        lines.push((total_seconds, text.to_string()));
    }

    lines
}

fn current_lyric(lines: &[(f64, String)], position: f64) -> Option<&str> {
    let mut current = None;

    for (timestamp, lyric) in lines {
        if *timestamp <= position {
            current = Some(lyric.as_str());
        } else {
            break;
        }
    }

    current
}

fn get_lyrics(
    client: &reqwest::blocking::Client,
    artist: &str,
    title: &str,
    album: &str,
    duration: f64,
) -> Option<Vec<(f64, String)>> {
    let url = format!(
        "https://lrclib.net/api/get?artist_name={}&track_name={}&album_name={}&duration={}",
        urlencoding::encode(artist),
        urlencoding::encode(title),
        urlencoding::encode(album),
        duration
    );

    let response = match client.get(url).send() {
        Ok(response) => response,
        Err(error) => {
            println!("Could not contact LRCLIB: {error}");
            return None;
        }
    };

    if !response.status().is_success() {
        println!("No lyrics found for this song.");
        return None;
    }

    let lyrics: LyricsResponse = match response.json() {
        Ok(lyrics) => lyrics,
        Err(error) => {
            println!("Could not read LRCLIB response: {error}");
            return None;
        }
    };

    match lyrics.synced_lyrics {
        Some(text) => Some(parse_lyrics(&text)),
        None => {
            println!("No synced lyrics found.");
            None
        }
    }
}

//kitty

fn clear_screen() {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

fn enter_lyrics_mode() {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    stdout.flush().unwrap();
}

fn leave_lyrics_mode() {
    let mut stdout = io::stdout();

    execute!(stdout, cursor::Show, LeaveAlternateScreen).unwrap();

    stdout.flush().unwrap();
}

// bigahh_text

fn big_text(text: &str) -> Vec<String> {
    let font = |c: char| -> [&str; 5] {
        match c.to_uppercase().next().unwrap_or(c) {
            // LATIN
            'A' => [" ███ ", "█   █", "█████", "█   █", "█   █"],
            'B' => ["████ ", "█   █", "████ ", "█   █", "████ "],
            'C' => [" ████", "█    ", "█    ", "█    ", " ████"],
            'D' => ["████ ", "█   █", "█   █", "█   █", "████ "],
            'E' => ["█████", "█    ", "████ ", "█    ", "█████"],
            'F' => ["█████", "█    ", "████ ", "█    ", "█    "],
            'G' => [" ████", "█    ", "█ ███", "█   █", " ████"],
            'H' => ["█   █", "█   █", "█████", "█   █", "█   █"],
            'I' => ["█████", "  █  ", "  █  ", "  █  ", "█████"],
            'J' => ["█████", "    █", "    █", "█   █", " ███ "],
            'K' => ["█   █", "█  █ ", "███  ", "█  █ ", "█   █"],
            'L' => ["█    ", "█    ", "█    ", "█    ", "█████"],
            'M' => ["█   █", "██ ██", "█ █ █", "█   █", "█   █"],
            'N' => ["█   █", "██  █", "█ █ █", "█  ██", "█   █"],
            'O' => [" ███ ", "█   █", "█   █", "█   █", " ███ "],
            'P' => ["████ ", "█   █", "████ ", "█    ", "█    "],
            'Q' => [" ███ ", "█   █", "█   █", "█  ██", " ████"],
            'R' => ["████ ", "█   █", "████ ", "█  █ ", "█   █"],
            'S' => [" ████", "█    ", " ███ ", "    █", "████ "],
            'T' => ["█████", "  █  ", "  █  ", "  █  ", "  █  "],
            'U' => ["█   █", "█   █", "█   █", "█   █", " ███ "],
            'V' => ["█   █", "█   █", "█   █", " █ █ ", "  █  "],
            'W' => ["█   █", "█   █", "█ █ █", "██ ██", "█   █"],
            'X' => ["█   █", " █ █ ", "  █  ", " █ █ ", "█   █"],
            'Y' => ["█   █", " █ █ ", "  █  ", "  █  ", "  █  "],
            'Z' => ["█████", "   █ ", "  █  ", " █   ", "█████"],

            //p-polish
            'Ą' => [" ███ ", "█   █", "█████", "█   █", "█  ██"],
            'Ć' => [" ████", "█    ", "█    ", "█    ", " ████"],
            'Ę' => ["█████", "█    ", "████ ", "█    ", "█████"],
            'Ł' => ["█    ", "█    ", "██   ", "█ █  ", "█████"],
            'Ń' => ["█   █", "██  █", "█ █ █", "█  ██", "█   █"],
            'Ó' => [" ███ ", "█   █", "█   █", "█   █", " ███ "],
            'Ś' => [" ████", "█    ", " ███ ", "    █", "████ "],
            'Ź' => ["█████", "   █ ", "  █  ", " █   ", "█████"],
            'Ż' => ["█████", "   █ ", "  █  ", " █   ", "█████"],

            // rusian:o
            'А' => [" ███ ", "█   █", "█████", "█   █", "█   █"],
            'Б' => ["█████", "█    ", "████ ", "█   █", "████ "],
            'В' => ["████ ", "█   █", "████ ", "█   █", "████ "],
            'Г' => ["█████", "█    ", "█    ", "█    ", "█    "],
            'Д' => ["  █  ", " ███ ", "█   █", "█████", "█   █"],
            'Е' => ["█████", "█    ", "████ ", "█    ", "█████"],

            'Ё' => ["█   █", "     ", "█████", "█    ", "████ "],

            'Ж' => ["█ █ █", " ███ ", "█████", " ███ ", "█ █ █"],
            'З' => ["████ ", "    █", " ███ ", "    █", "████ "],
            'И' => ["█   █", "██  █", "█ █ █", "█  ██", "█   █"],
            'Й' => ["█ █ █", "█   █", "██  █", "█ █ █", "█  ██"],

            'К' => ["█   █", "█  █ ", "███  ", "█  █ ", "█   █"],
            'Л' => ["  ██ ", " █ █ ", "█  █ ", "█   █", "█   █"],
            'М' => ["█   █", "██ ██", "█ █ █", "█   █", "█   █"],
            'Н' => ["█   █", "█   █", "█████", "█   █", "█   █"],
            'О' => [" ███ ", "█   █", "█   █", "█   █", " ███ "],
            'П' => ["█████", "█   █", "█   █", "█   █", "█   █"],
            'Р' => ["████ ", "█   █", "████ ", "█    ", "█    "],
            'С' => [" ████", "█    ", "█    ", "█    ", " ████"],
            'Т' => ["█████", "  █  ", "  █  ", "  █  ", "  █  "],
            'У' => ["█   █", " █ █ ", "  █  ", " █   ", "█    "],
            'Ф' => ["  █  ", " ███ ", "█ █ █", " ███ ", "  █  "],
            'Х' => ["█   █", " █ █ ", "  █  ", " █ █ ", "█   █"],
            'Ц' => ["█   █", "█   █", "█   █", "█  ██", "█████"],
            'Ч' => ["█   █", "█   █", "█████", "    █", "    █"],
            'Ш' => ["█ █ █", "█ █ █", "█ █ █", "█ █ █", "█████"],
            'Щ' => ["█ █ █", "█ █ █", "█ █ █", "█ ███", "█████"],
            'Ъ' => ["███  ", "  █  ", "████ ", "█   █", "████ "],
            'Ы' => ["█   █", "█   █", "██ ██", "█   █", "█   █"],
            'Ь' => ["█    ", "█    ", "████ ", "█   █", "████ "],
            'Э' => [" ████", "    █", " ████", "    █", " ████"],
            'Ю' => ["█ ███", "█ █  ", "█████", "█ █  ", "█ ███"],
            'Я' => [" ████", "█   █", " ████", "   █ ", "█  █ "],
            // NUMBERS
            '0' => [" ███ ", "█  ██", "█ █ █", "██  █", " ███ "],
            '1' => ["  █  ", " ██  ", "  █  ", "  █  ", "█████"],
            '2' => [" ███ ", "█   █", "   █ ", " █   ", "█████"],
            '3' => ["████ ", "    █", " ███ ", "    █", "████ "],
            '4' => ["█  █ ", "█  █ ", "█████", "   █ ", "   █ "],
            '5' => ["█████", "█    ", "████ ", "    █", "████ "],
            '6' => [" ███ ", "█    ", "████ ", "█   █", " ███ "],
            '7' => ["█████", "    █", "   █ ", "  █  ", " █   "],
            '8' => [" ███ ", "█   █", " ███ ", "█   █", " ███ "],
            '9' => [" ███ ", "█   █", " ████", "    █", " ███ "],

            // PUNCTUATION
            '!' => ["  █  ", "  █  ", "  █  ", "     ", "  █  "],
            '?' => [" ███ ", "█   █", "   █ ", "     ", "  █  "],
            '.' => ["     ", "     ", "     ", "     ", "  █  "],
            ',' => ["     ", "     ", "     ", "  █  ", " █   "],
            '\'' => ["  █  ", "  █  ", "     ", "     ", "     "],
            '-' => ["     ", "     ", "█████", "     ", "     "],
            ':' => ["     ", "  █  ", "     ", "  █  ", "     "],

            //space
            ' ' => ["     ", "     ", "     ", "     ", "     "],

            //unknownchar
            _ => ["     ", "     ", "     ", "     ", "     "],
        }
    };

    let mut output = vec![String::new(); 5];

    for c in text.chars() {
        let character = font(c);

        for row in 0..5 {
            output[row].push_str(character[row]);
            output[row].push(' ');
        }
    }

    output
}

fn display_big_lyrics(lyric: &str) {
    let mut stdout = io::stdout();

    let (width, height) = terminal::size().unwrap();

    let text = if lyric.is_empty() {
        "NO LYRICS MATCHED"
    } else {
        lyric
    };

    let words: Vec<&str> = text.split_whitespace().collect();

    // Each big-font character is roughly 6 terminal columns wide.
    let max_chars = (width as usize / 6).max(1);

    let mut wrapped_lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        let new_length = if current_line.is_empty() {
            word.chars().count()
        } else {
            current_line.chars().count() + 1 + word.chars().count()
        };

        if new_length > max_chars && !current_line.is_empty() {
            wrapped_lines.push(current_line);
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
            }

            current_line.push_str(word);
        }
    }

    if !current_line.is_empty() {
        wrapped_lines.push(current_line);
    }

    let mut rendered_lines = Vec::new();

    for line in wrapped_lines {
        rendered_lines.extend(big_text(&line));
        rendered_lines.push(String::new());
    }

    if rendered_lines.len() > height as usize {
        rendered_lines.truncate(height as usize);
    }

    let start_y = height.saturating_sub(rendered_lines.len() as u16) / 2;

    for (index, line) in rendered_lines.iter().enumerate() {
        let line_width = line.chars().count() as u16;
        let x = width.saturating_sub(line_width) / 2;

        execute!(stdout, cursor::MoveTo(x, start_y + index as u16)).unwrap();

        print!("{line}");
    }

    stdout.flush().unwrap();
}

//main

fn main() {
    let args = cli::Args::parse();

    let mode = if args.lyrics {
        cli::DisplayMode::Lyrics
    } else if args.info {
        cli::DisplayMode::Info
    } else if args.time {
        cli::DisplayMode::Time
    } else {
        cli::DisplayMode::Full
    };

    if let cli::DisplayMode::Lyrics = mode {
        enter_lyrics_mode();

        ctrlc::set_handler(|| {
            leave_lyrics_mode();
            std::process::exit(0);
        })
        .unwrap();
    }

    let finder = match PlayerFinder::new() {
        Ok(finder) => finder,

        Err(error) => {
            println!("Could not connect to MPRIS: {error}");
            return;
        }
    };

    let player = match finder.find_active() {
        Ok(player) => player,

        Err(error) => {
            println!("Could not find an active music player: {error}");
            return;
        }
    };

    let client = match reqwest::blocking::Client::builder()
        .user_agent("lrcly/0.1.0")
        .build()
    {
        Ok(client) => client,

        Err(error) => {
            println!("Could not create HTTP client: {error}");
            return;
        }
    };

    let mut current_song = String::new();

    let mut current_artist = String::new();
    let mut current_title = String::new();
    let mut current_album = String::new();
    let mut current_duration = 0.0;

    let mut lines: Vec<(f64, String)> = Vec::new();
    let mut last_lyric = String::new();

    loop {
        let metadata = match player.get_metadata() {
            Ok(metadata) => metadata,

            Err(error) => {
                println!("Could not get player metadata: {error}");
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };

        let title = match metadata.title() {
            Some(title) => title.to_string(),

            None => {
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };

        let artist = match metadata.artists() {
            Some(artists) => artists.join(", "),

            None => {
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };

        let album = metadata.album_name().unwrap_or("").to_string();

        let duration = match metadata.length() {
            Some(duration) => duration.as_secs_f64(),
            None => 0.0,
        };

        let song_id = format!("{artist}::{title}");

        if song_id != current_song {
            lines = match get_lyrics(&client, &artist, &title, &album, duration) {
                Some(lines) => lines,
                None => Vec::new(),
            };

            current_song = song_id;

            current_artist = artist;
            current_title = title;
            current_album = album;
            current_duration = duration;

            last_lyric.clear();
            clear_screen();
        }

        let position = match player.get_position() {
            Ok(position) => position.as_secs_f64(),

            Err(error) => {
                println!("Could not get playback position: {error}");
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };

        let lyric = current_lyric(&lines, position).unwrap_or("");

        let should_render = lyric != last_lyric
            || (matches!(mode, cli::DisplayMode::Lyrics) && last_lyric.is_empty());

        if should_render {
            match mode {
                cli::DisplayMode::Lyrics => {
                    clear_screen();
                    display_big_lyrics(lyric);
                }

                cli::DisplayMode::Info => {
                    clear_screen();

                    println!("{} - {}", current_artist, current_title);
                    println!("Album: {}", current_album);
                    println!();
                    println!("{lyric}");
                }

                cli::DisplayMode::Time => {
                    clear_screen();

                    println!("{lyric}");
                    println!();

                    println!(
                        "{:02}:{:02} / {:02}:{:02}",
                        (position as u64) / 60,
                        (position as u64) % 60,
                        (current_duration as u64) / 60,
                        (current_duration as u64) % 60
                    );
                }

                cli::DisplayMode::Full => {
                    clear_screen();

                    println!("Now playing:");
                    println!("{} - {}", current_artist, current_title);
                    println!("Album: {}", current_album);
                    println!();
                    println!("{lyric}");
                    println!();

                    println!(
                        "{:02}:{:02} / {:02}:{:02}",
                        (position as u64) / 60,
                        (position as u64) % 60,
                        (current_duration as u64) / 60,
                        (current_duration as u64) % 60
                    );
                }
            }

            io::stdout().flush().unwrap();

            last_lyric = lyric.to_string();
        }

        thread::sleep(Duration::from_millis(500));
    }
}
