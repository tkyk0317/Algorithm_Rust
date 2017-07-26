// 挿入ソート.
pub fn sort<T: PartialOrd + Clone>(a: &mut Vec<T>) {
    for i in 1..a.len() {
        let mut j = i;
        while 0 < j && a[j - 1] > a[j] {
            let _t = a[j].clone();
            a[j] = a[j - 1].clone();
            a[j - 1] = _t;
            j -= 1;
        }
    }
}
