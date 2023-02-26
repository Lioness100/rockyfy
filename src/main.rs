use rodio::{Decoder, OutputStream, Sink};
use std::{error::Error, io::Cursor, process::Command, thread, time::Duration, process, env};

fn main() -> Result<(), Box<dyn Error>> {
    let command = env::args().skip(1).collect::<Vec<_>>().join(" ");

    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let cursor = Cursor::new(include_bytes!("../assets/Gonna Fly Now.mp3"));
    sink.append(Decoder::new_looped(cursor)?);

    let exit_code = Command::new("sh").arg("-c").arg(&command).spawn()?.wait()?;

    for volume in (0..=100).rev() {
        sink.set_volume(volume as f32 / 100.0);
        thread::sleep(Duration::from_millis(20));
    }

    process::exit(exit_code.code().unwrap_or(1));
}
