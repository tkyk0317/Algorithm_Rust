extern crate rand;

use rand::Rng;
use std::time::{Duration, Instant};

mod sort;

// 計測マクロ.
macro_rules! performance {
  ($t:tt, $x:expr) => {
    {
      let start = Instant::now();
      let result = $x;
      let end = start.elapsed();
      println!("{}{}.{:06} sec", $t, end.as_secs(), end.subsec_nanos() / 1_000_000);
      result
    }
  };
}

// main function.
fn main() {
    let mut a = vec![];
    for _ in 0..100000 { a.push(rand::thread_rng().gen::<u8>()); }
    // sort.
    performance!("Rust Standard Sort: ", { a.clone().sort(); });
    performance!("Insert Sort       : ", { sort::insert::sort(&mut a.clone()) });
    performance!("Select Sort       : ", { sort::select::sort(&mut a.clone()); });
}
