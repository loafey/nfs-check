use std::{io, path::Path, time::Instant};

fn thread_worker<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let dir = std::fs::read_dir(path)?;
    if dir.count() > 0 {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "no data"))
    }
}

fn main() {
    let folder = std::env::args().nth(1).unwrap();

    let t = std::thread::spawn(move || thread_worker(folder).unwrap());
    let now = Instant::now();
    loop {
        if now.elapsed().as_secs_f32() > 1.0 {
            std::process::exit(1);
        } else if t.is_finished() {
            if t.join().is_err() {
                std::process::exit(1)
            } else {
                break;
            }
        }
    }
}
