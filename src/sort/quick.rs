use std::fmt::Debug;

// クイックソート.
pub fn sort<T: Ord+Clone+Debug>(a: &mut Vec<T>) {
    quick(0, a.len() - 1, a);
}

// 挿入ソート.
// 要素数が少なくなった場合、クイック→挿入へ切り替える.
fn insert_sort<T: Ord+Clone>(left: usize, right: usize, data: &mut Vec<T>) {
    for i in (left + 1)..(right + 1) {
        let mut j = i;
        while left < j && data[j - 1] > data[j] {
            let _t = data[j].clone();
            data[j] = data[j - 1].clone();
            data[j - 1] = _t;
            j -= 1;
        }
    }
}

// クイックソート本体.
fn quick<T: Ord+Clone+Debug>(left: usize, right: usize, data: &mut Vec<T>) {
    // 左右の位置がおかしい場合、リターン.
    if left >= right { return; }

    // 要素数が少ない場合、挿入ソートに切り替える（最適化）.
    if right - left < 10 {
        insert_sort(left, right, data);
    }
    else {
        // クイックソート.
        let mut l = left;
        let mut r = right;
        let pivot = data[(left + right) >> 1].clone();
        loop {
            // 左から小さい値、右から大きい値を検索し、交換.
            while data[l] < pivot { l += 1; }
            while pivot < data[r] { r -= 1; }

            // 追い越したらbreak.
            if l >= r { break; }

            // データ交換.
            data.swap(r, l);
            r -= 1; l += 1;
        }
        // 左/右側の配列をクイックソート.
        if left + 1 < l { quick(left, l - 1, data); } // usize型(l)のアンダフローに気をつける.
        if r + 1 < right { quick(r + 1, right, data); }
    }
}