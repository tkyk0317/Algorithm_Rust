// バブルソート..
pub fn sort<T: Ord>(data: &mut Vec<T>) {
    let len = data.len();
    let mut end = len;
    while end > 0 {
        for i in 0..data.len() - 1 {
            // 隣同士の要素を比較＆入れ替え.
            if data[i] > data[i + 1] { data.swap(i, i+ 1); }
        }
        end -= 1;
    }
}