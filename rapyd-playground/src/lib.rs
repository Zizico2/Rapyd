pub mod app;

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

mod counter;
