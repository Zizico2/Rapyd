mod hello_world_kj;
use bytecheck::CheckBytes;
use hello_world_kj::component::Component;
use rapyd_macros::do_nothing;
use rkyv::{with, Archive, Deserialize, Serialize};
use std::{fs::remove_file, fs::OpenOptions, io::Write};
use std::{path::Path, slice};
mod component;
mod counter;

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

#[derive(Clone, Copy)]
pub enum Walk {
    // go n levels deeper (the bool represents weather,
    // the node we end up inside of has events or not)
    Next(usize, bool),
    // skip n nodes
    Over(usize),
    // go n levels shallower
    Out(usize),
    // replace the next node. Doesn't move forward.
    // if you were to do Replace again, you would replace the newly inserted node
    //
    // if yo u were to do FlagAsEventTarget after Replace you would be flagging
    // the newly inserted node
    Replace,
    // flag the next node as an event target. Doesn't move forward.
    EventTarget,
    // an array of walks. This Variant will be used inside other arrays of walks,
    // creating a tree-like structure. These trees are meant to be read as if they were flat.
    MoreWalks(&'static [Walk]),
}

impl<'a> IntoIterator for &'a Walk {
    type Item = &'a Walk;

    type IntoIter = slice::Iter<'a, Walk>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Walk::MoreWalks(arr) => arr.into_iter(),
            default => default.into_iter()
        }
    }
}
