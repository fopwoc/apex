use apex_client::run;
use pollster::FutureExt;

fn main() {
    run().block_on();
}
