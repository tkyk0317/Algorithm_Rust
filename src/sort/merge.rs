use std::fmt::Debug;

// マージソート本体.
pub fn sort<T: Ord+Clone+Debug>(data: &mut Vec<T>) {
    let mut src = data.clone();
    let end = data.len();
    merge_sort(&mut src, data, 0, end);
}

// マージソート実処理部.
fn merge_sort<T: Ord+Clone+Debug>(work: &mut Vec<T>, result: &mut Vec<T>, start: usize, end: usize) {
    // 要素数が１つになればリターン.
    if end - start < 2 { return; }

    // 要素が２つしか存在しない場合、入れ替える.
    if end - start == 2 {
        if result[start + 1] < result[start] {
            result.swap(start, start + 1);
            return;
        }
    }

    // 領域を半分に分割し、其々をソート.
    let mid = (start + end) / 2;
    merge_sort(result, work, start, mid);
    merge_sort(result, work, mid, end);

    let mut idx = start;
    let mut left = start;
    let mut right = mid;
    while idx < end {
        // 右の要素のほうが大きい、もしくは左側に要素が無い場合、右側から取得.
        if left >= mid || (right < end && work[left] > work[right]) {
            result[idx] = work[right].clone();
            right += 1;
        }
        else {
            result[idx] = work[left].clone();
            left += 1;
        }
        idx += 1;
    }
}