//use partial order to support floats.
//copy or clone needed for .to_vec()

pub fn merge_sort<T: PartialOrd + Copy>(items: &mut [T]) {

    if items.len() == 0 {
        return;
    }

    if items.len() > 1 {
            let mid = items.len()/2;

            merge_sort(&mut items[..mid]);
            merge_sort(&mut items[mid..]);
        
            merge(items, mid);
    }
}

fn merge<T: PartialOrd + Copy>(items: &mut [T], mid: usize) {
    //copy the left and right sides of the array
    let left_array = items[..mid].to_vec();
    let right_array = items[mid..].to_vec();
    
    let mut l_idx:usize = 0;
    let mut r_idx:usize = 0;


    for idx in 0..items.len() {
        //while in bounds of each array
        if l_idx < left_array.len() && r_idx < right_array.len() {
            if left_array[l_idx] < right_array[r_idx] {
                items[idx] = left_array[l_idx];
                l_idx += 1;
            } else {
                items[idx] = right_array[r_idx];
                r_idx += 1;
            }
        } else {
            //finally check both left range and right range and add remain to the end...
            if l_idx < left_array.len() {
                items[idx] = left_array[l_idx];
                l_idx += 1;
            }
            
            if r_idx < right_array.len() {
                items[idx] = right_array[r_idx];
                r_idx += 1;
            }
        }
    }
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

        //to see output run: cargo test -- --nocapture
        println!("{:?}", items);

        for idx in 0..(100 - 2) {
            assert!(items[idx] <= items[idx + 1]);
        }
    }
}