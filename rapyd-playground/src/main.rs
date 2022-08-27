mod hello_world_kj;
use bytecheck::CheckBytes;
use hello_world_kj::component::Component;
use rapyd_macros::do_nothing;
use rkyv::{with, Archive, Deserialize, Serialize};
use std::{fs::remove_file, fs::OpenOptions, io::Write};
use std::{path::Path, slice};

fn main() -> std::io::Result<()> {
    if Path::new("./hello_world_kj.html").exists() {
        remove_file("./hello_world_kj.html")?;
    }
    let comp = Component::new();
    let res = comp.render();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true) // <--------- this
        .create(true)
        .open("./hello_world_kj.html")?;

    file.write_all(res.as_bytes())?;
    file.flush()?;

    Ok(())
}
