use std::sync::atomic::AtomicUsize;

#[repr(C)]
struct RingBufferHeader {
    /// writer(s) synchronization
    write_position: AtomicUsize,
    /// sample rate
    sample_rate: u32,
    /// mono = 1; stereo = 2;
    channels: u16,
    /// buffer capacity in bytes
    capacity: usize,
    /// chunk size in bytes
    chunk_size: usize,
}
