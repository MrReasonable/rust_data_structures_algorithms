use std::fmt::Debug;

use crate::b_rand;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    let mut sorted = true;
    for p in 0..v.len() {
        for i in 0..(v.len()) - 1 - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false
            }
        }
        println!("v: {:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    println!("MS: {:?}", v);
    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    'sort: loop {
        println!("a: {:?}, b: {:?}, res: {:?}", a_peek, b_peek, res);
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    break 'sort;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                break 'sort;
            }
        }
    }

    println!("Sorted: {:?}", res);

    res
}

pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len() - 1);
    println!("{}", p);
    v.swap(p, 0);
    p = 0;
    println!("{:?}", v.len());
    for i in 1..v.len() {
        if v[i] < v[p] {
            println!("i: {}, p: {}, v.len: {:?}", i, p, v.len());
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort_rayon<T: Send + PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    println!("{:?}", v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 13, 1, 8, 11, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 13, 1, 8, 11, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 8, 3, 13, 19, 11, 1];
        let p = pivot(&mut v);
        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 8, 3, 13, 19, 11, 1];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);
    }

    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![4, 6, 8, 3, 13, 19, 11, 1];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13, 19]);
    }
}
