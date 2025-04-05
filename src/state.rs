pub struct State {
    pub ad_index: usize,
    pub best_block_time: u64,
    pub is_exit_request: bool,
    pub iteration_total: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            ad_index: 0,
            best_block_time: 0,
            is_exit_request: false,
            iteration_total: 0,
        }
    }
}
