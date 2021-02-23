use crate::communication::messaging;
use crate::protocol;

pub struct Receiver {
    state: messaging::State,
    section: messaging::Section,
    index: i32,

    header: protocol::Header,
}
