use macroquad::prelude as mq;

const TICKS_PER_SECOND: usize = 120;
const TICK_LEN_SECONDS: f64 = 1. / TICKS_PER_SECOND as f64;
#[macroquad::main("Fixed Timestep")]
async fn main() {
    let mut ticks_so_far = 0;
    loop {
        let time = mq::get_time();
        let expected_tick_count = (time / TICK_LEN_SECONDS).floor() as usize;
        let ticks_to_perform = expected_tick_count - ticks_so_far;
        for _ in 0..(ticks_to_perform + 1) {
            // tick logic here
        }
        ticks_so_far += ticks_to_perform;
        mq::clear_background(mq::BLACK);
        mq::next_frame().await
    }
}
