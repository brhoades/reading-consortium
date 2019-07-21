#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;


fn main() {
}

fn merge_sort<T: PartialOrd>(slc: &mut [T]) {
    if slc.len() <= 1 {
        return
    }

    merge_inner(slc, 0, slc.len() - 1)
}

fn merge_inner<T: PartialOrd>(slc: &mut [T], left_i: usize, right_i: usize) {
    let middle = ((right_i as f64 - left_i as f64) / 2.0).ceil() as usize + left_i;
    let mut i = left_i;
    let mut j = middle;

    if right_i - left_i >= 2 {
        merge_inner(slc, left_i, middle);
        merge_inner(slc, middle + 1, right_i);
        j += 1;
    }

    while j <= right_i {
        if i == j {
            // out of values to compare on lhs.
            j += 1;
        } else if slc[i] > slc[j] {
            // rotate first unmerged value of j and all other values of i left.
            // this sorts the value of 1 in the spot properly.
            // eg: [2,3,5,1,2]; i = 0, j = 3.
            //          ^ middle
            //     rotates [2,3,5,1] left => [1,2,3,5].
            //     giving [1,2,3,5,2]; i = 1; j = 4
            slc[i..=j].rotate_left(j - i);
            i += 1;
            if j < right_i {
                j += 1
            }
        } else {
            // Value is sorted, advance i.
            i += 1;
        }
    }
}

#[test]
fn test_sort_sorted() {
    let mut arr: [u8; 0] = [];
    let mut arr_1 = [1];
    let mut arr_2 = [1,2];
    let mut arr_3 = [1,2,3];
    let mut arr_5 = [1,2,3,4,5];

    merge_sort(&mut arr[0..0]);

    merge_sort(&mut arr_1[0..1]);
    assert_eq!(arr_1, [1]);

    merge_sort(&mut arr_2[0..2]);
    assert_eq!(arr_2, [1,2]);

    merge_sort(&mut arr_3[0..3]);
    assert_eq!(arr_3, [1,2,3]);

    merge_sort(&mut arr_5[0..5]);
    assert_eq!(arr_5, [1,2,3,4,5]);
}

#[test]
fn test_sort_unsorted() {
    for x in 2..10i64 {
        let mut arr: Vec<i64> = (0..x).rev().collect();
        let corr: Vec<i64> = (0..x).collect();

        merge_sort(&mut arr[0..(x as usize)]);
        assert_eq!(arr, corr);
    }

    let mut arr = vec!(92,1,20,91,200,100,200,109,19,0);
    merge_sort(&mut arr[0..10]);

    assert_eq!(arr, vec!(0,1,19,20,91,92,100,109,200,200));
}

#[cfg(test)]
#[quickcheck]
fn merge_sort_is_rust_sort(xs: Vec<isize>) -> bool {
    let mut sorted = xs.clone();
    let mut ours = xs.clone();
    let size = xs.len();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    merge_sort(&mut ours[0..size]);

    for x in 0..size {
        if ours[x] != sorted[x] {
            return false
        }
    }

    true
}
