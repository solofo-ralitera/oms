use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    let path = "/media/solofo/MEDIA/films/Adaline/Adaline.avi";

    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, path).unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
 
}