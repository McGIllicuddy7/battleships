use crate::utils::Fx;

pub mod io;
pub mod parser;
pub mod ship;
pub mod utils;
pub fn main() {
    let mut total_diff = 0.0;
    let count = 6280000;
    let mut max_diff = 0.0;
    let mut cx = 0;
    for i in -count..=count {
        let a = Fx::new(i) / Fx::new(10000);
        if a.cos() == Fx::new(0) {
            continue;
        }
        cx += 1;
        let asin = a.tan();
        let af64sin = (i as f64 / 10000.).tan();
        let diff = (asin.get_f64() - af64sin).abs();
        if diff > max_diff {
            max_diff = diff;
        }
        total_diff += diff;
        if diff > 100. {
            println!(
                "i:{}, sin(i/10000):{} f64 sin(i/10000):{}, diff:{}",
                a, asin, af64sin, diff
            );
        }
    }
    total_diff /= cx as f64;
    println!("average difference:{total_diff}");
    println!("max difference:{max_diff}");
}
