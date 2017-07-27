extern crate rand;

use rand::Rng;
use std::time::{Instant};

mod sort;

// 計測マクロ.
macro_rules! performance {
  ($t:tt, $x:expr) => {
    {
      let start = Instant::now();
      let result = $x;
      let end = start.elapsed();
      println!("[{}] {}.{:09} sec", $t, end.as_secs(), end.subsec_nanos());
      result
    }
  };
}

// main function.
fn main() {
    let mut a = vec![];
    for _ in 0..10000 { a.push(rand::thread_rng().gen::<u16>()); }
    //let a = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut r = a.clone();
    r.sort();

    println!("---------------------------------------");
    println!("[sort] Item Count: {}", a.len());
    println!("---------------------------------------");

    // sort.
    let mut _standard = a.clone();
    performance!("Vec::sort  ", { _standard.sort(); });
    assert_eq!(_standard, r);

    let mut _insert = a.clone();
    performance!("Insert Sort", { sort::insert::sort(&mut _insert); });
    assert_eq!(_insert, r);

    let mut _select = a.clone();
    performance!("Select Sort", { sort::select::sort(&mut _select); });
    assert_eq!(_select, r);

    let mut _heap = a.clone();
    performance!("Heap Sort  ", { sort::heap::sort(&mut _heap); });
    assert_eq!(_heap, r);

    let mut _quick = a.clone();
    performance!("Quick Sort ", { sort::quick::sort(&mut _quick); });
    assert_eq!(_quick, r);
}
