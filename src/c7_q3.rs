// Jukebox: Design a musical jukebox using object-oriented principles.

use std::collections::VecDeque;

trait JukeBox {
    fn new() -> Self;
    fn play(&mut self, song: Song);
    fn browse_artist(&self, artist: String) -> Vec<Song>;
    fn currently_playing(&self) -> Vec<&Song>;
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Song {
    name: String,
    artist: String,
}

#[derive(Debug)]
struct JB {
    playing: VecDeque<Song>,
    songs: Vec<Song>,
}

impl JukeBox for JB {
    fn new() -> Self {
        JB {
            playing: VecDeque::new(),
            songs: vec![
                Song {
                    name: "Big Empty".to_string(),
                    artist: "STP".to_string(),
                },
                Song {
                    name: "Piece of Pie".to_string(),
                    artist: "STP".to_string(),
                },
                Song {
                    name: "Not Enough Time".to_string(),
                    artist: "INXS".to_string(),
                },
            ],
        }
    }

    fn play(&mut self, song: Song) {
        self.playing.push_front(song)
    }

    fn browse_artist(&self, artist: String) -> Vec<Song> {
        self.songs
            .iter()
            .filter(|s| s.artist == artist)
            .cloned()
            .collect()
    }

    fn currently_playing(&self) -> Vec<&Song> {
        self.playing.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_play_songs() {
        let mut jb = JB::new();
        let mut songs = jb.browse_artist("STP".to_string());
        let song = songs.pop().unwrap();
        jb.play(song.clone());

        let playing = jb.currently_playing()[0];
        assert_eq!(song, *playing);
    }
}
