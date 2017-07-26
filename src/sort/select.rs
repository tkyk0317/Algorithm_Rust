// 選択ソート.
pub fn sort<T: Ord>(a: &mut Vec<T>) {
    let mut n = a.len() - 1;
    while n > 0 {
        // 最大値を検索.
        let max_index: usize = a.split_at(n + 1)
                                .0
                                .iter()
                                .enumerate()
                                .max_by(|x, y| x.1.cmp(y.1))
                                .unwrap().0;

        // 最大値を最終要素と入れ替える.
        a.swap(max_index, n);
        n -= 1;
    }
}