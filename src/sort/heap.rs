// ヒープソート.
pub fn sort<T: Ord+Clone>(a: &mut Vec<T>) {
    let mut len = a.len();
    while len > 0 {
        // ヒープを作成し、先頭と最後の要素を入れ替える.
        create_heap(len, a);
        a.swap(0, len - 1);
        len -= 1;
    }
}

// ヒープ作成.
fn create_heap<T: Ord>(len: usize, a: &mut Vec<T>) {
    let mut p = len / 2;
    loop {
        let mut s = p;
        while s < len {
            let mut max_idx = 2 * s + 1; // 左ノード要素のほうが、親ノードより大きいと仮定.

            // 右子要素を交換できるかチェック.
            if max_idx + 1 < len && a[max_idx + 1] > a[max_idx] { max_idx += 1; }

            // 範囲チェック.
            if max_idx >= len || a[s] >= a[max_idx] { break; }

            // 交換.
            a.swap(max_idx, s);
            s = max_idx;
        }
        // 先頭要素を処理したらループを抜ける.
        if p == 0 { break; }
        p -= 1;
    }
}