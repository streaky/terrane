pub enum Event {
    Unsupported(*const u8, *const u8),
    Ready,
}
