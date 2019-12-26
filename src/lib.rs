const MAXIMUM_CAPACITY: usize = 1 << 30;
const DEFAULT_CAPACITY: usize = 16;
const LOAD_FACTOR: f64 = 0.75;
const MIN_TRANSFER_STRIDE: usize = 16;
const RESIZE_STAMP_BITS: usize = 16;
const MAX_RESIZERS: usize = (1 << (32 - RESIZE_STAMP_BITS)) -1;
const RESIZE_STAMP_SHIFT: usize = 32 - RESIZE_STAMP_BITS;

mod node;