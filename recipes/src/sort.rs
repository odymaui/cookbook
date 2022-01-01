//use partial order to support floats.
//copy or clone needed for .to_vec()
pub fn merge_sort<T: PartialOrd + Copy>(items: &mut Vec<T>) {
    //handle bounds interally    
    if items.len() == 0 {
        return;
    }

    let left: usize = 0;
    let right: usize = items.len() - 1;

    merge_sort_internal(items, left, right);
}

fn merge_sort_internal<T: PartialOrd + Copy>(items: &mut Vec<T>, left: usize, right: usize) {

    let mid: usize;

    if left < right {
        mid = ((right - left) / 2) + left;
        if mid >= left && mid <= right {
            merge_sort_internal(items, left, mid);
            merge_sort_internal(items, mid + 1, right);
        
            merge(items, left, mid, right);
        }
    }
}

fn merge<T: PartialOrd + Copy>(items: &mut Vec<T>, left: usize, mid: usize, right: usize) {
    //copy the left and right sides of the array
    let left_array = copy_array(items, left, mid);
    let right_array = copy_array(items, mid + 1, right);
    
    let mut leftidx:usize = 0;
    let mut rightidx:usize = 0;
    let mut mainidx = left;
    //guard the bounds access
    let idx_chk = |leftidx, rightidx| left_array.len() > leftidx && right_array.len() > rightidx;

    //loop over array range length while left and right are not done
    while idx_chk(leftidx, rightidx) && left_array.len() > 0 && right_array.len() > 0 {
        //compare left to right and swap as needed
        if idx_chk(leftidx, rightidx) && left_array[leftidx] <= right_array[rightidx] {
            items[mainidx] = left_array[leftidx];
            leftidx += 1;
            mainidx += 1;
        }

        if idx_chk(leftidx, rightidx) && right_array[rightidx] < left_array[leftidx] {
            items[mainidx] = right_array[rightidx];
            rightidx += 1;
            mainidx += 1;
        }
    }

    //finally check both left range and right range and add remain to the end...
    while leftidx < left_array.len() {
        items[mainidx] = left_array[leftidx];
        leftidx += 1;
        mainidx += 1;
    }
    
    while rightidx < right_array.len() {
        items[mainidx] = right_array[rightidx];
        rightidx += 1;
        mainidx += 1;
    }
}

fn copy_array<T: PartialOrd + Copy>(items: &Vec<T>, left: usize, mid: usize) -> Vec<T> {
    items[left..=mid].to_vec()
}


#[cfg(test)]
mod sort_tests {
    #[test]
    fn sort_empty() {
        let mut items: Vec<u64> = vec!();

        super::merge_sort(&mut items);

        assert!(items.len() == 0);
    }

    #[test]
    fn sort_one() {
        let a = 42;

        let mut items: Vec<u64> = vec!(a);

        super::merge_sort(&mut items);

        assert!(items.len() == 1);
        assert!(a == items[0]);

    }

    #[test]
    fn sort_is_ordered() {
        let mut items: Vec<u64> = vec!(10,9,8,7,6,5,4,3,2,1,10,16,8,4,2,1);
        let l = items.len();
        super::merge_sort(&mut items);

        assert!(items.len() == l);
        for idx in 0..l-2 {
            assert!(items[idx] <= items[idx + 1]);
        }
        //to see output run: cargo test -- --nocapture
        println!("{:?}", items);
    }

    #[test]
    fn sort_is_ordered_floats() {
        let mut items: Vec<f64> = vec!(10.1,9.1,8.1,7.1,6.1,5.1,4.1,3.1,2.1,1.1,10.1,16.1,8.1,4.1,2.1,1.1);
        let l = items.len();
        super::merge_sort(&mut items);

        assert!(items.len() == l);
        for idx in 0..l-2 {
            assert!(items[idx] <= items[idx + 1]);
        }
        //to see output run: cargo test -- --nocapture
        println!("{:?}", items);
    }

    #[test]
    fn sort_is_randomly_ordered() {
        use rand::prelude::*;

        let mut rng = rand::thread_rng();
        
        let mut items: Vec<u64> = (1..100).collect();
        items.shuffle(&mut rng);

        println!("{:?}", items);

        super::merge_sort(&mut items);

        for idx in 0..(100 - 2) {
            assert!(items[idx] <= items[idx + 1]);
        }
        //to see output run: cargo test -- --nocapture
        println!("{:?}", items);
    }
}