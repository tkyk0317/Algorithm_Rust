extern crate rand;
extern crate libc;

use rand::Rng;
use std::time::{Instant};
use std::cmp::Ordering;
use libc::{size_t, c_void, c_int};

mod sort;

// libcライブラリをリンク.
#[link(name = "c")]
extern {
    fn qsort(data: *mut c_void,
             num: size_t,
             size: size_t,
             comp: extern fn(*const c_void, *const c_void) -> c_int);
}

// qsortコールバック関数.
extern fn _compare<T: Ord>(a: *const T, b: *const T) -> c_int {
    match unsafe { (*a).cmp(&*b) } {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0
    }
}

// qsort。型安全にする為、ラッパーを書く.
fn c_qsort<T: Ord>(data: &mut [T]) {
    // transmuteでキャスト.
    let _ptr: extern fn(*const c_void, *const c_void) -> c_int
        = unsafe { std::mem::transmute(_compare::<T> as (extern fn(*const T, *const T) -> c_int)) };
    unsafe {
        qsort(data.as_mut_ptr() as *mut c_void,
              data.len() as size_t,
              std::mem::size_of::<T>() as size_t,
              _ptr);
    }
}

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
    let mut count: i32 = 5000;
    loop {
        if count > 1000000 { break; }

        let mut a = vec![];
        for _ in 0..count { a.push(rand::thread_rng().gen::<u16>()); }
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

        let mut _c_qsort = a.clone();
        performance!("C qsort    ", { c_qsort(&mut _c_qsort); });
        assert_eq!(_c_qsort, r);

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

        let mut _bubble = a.clone();
        performance!("Bubble Sort", { sort::bubble::sort(&mut _bubble); });
        assert_eq!(_bubble, r);

        let mut _merge = a.clone();
        performance!("Merge Sort ", { sort::merge::sort(&mut _merge); });
        assert_eq!(_merge, r);
        count = count + 5000;
    }
}
