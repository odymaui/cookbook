pub fn merge_sort(items: &mut Vec<u64>) {//-> &mut Vec<u64> {
    
    if items.len() == 0 {
        return;
    }
    //find midpoint
    let left: usize = 0;
    let right: usize = items.len() - 1;

    merge_sort_int(items, left, right);
}

fn merge_sort_int(items: &mut Vec<u64>, left: usize, right: usize) {//-> &mut Vec<u64> {

    let mid: usize;

    if left < right {
        mid = ((right - left) / 2) + left;
        if mid >= left && mid <= right {
            merge_sort_int(items, left, mid);
            merge_sort_int(items, mid + 1, right);
        
            merge(items, left, mid, right);
        }
    }
}


fn merge(items: &mut Vec<u64>, left: usize, mid: usize, right: usize) {//-> &mut Vec<u64> {

    let left_array = get_array(items, left, mid);
    let right_array = get_array(items, mid + 1, right);
    
    //overall position within range, 0 is start for each copied array
    let mut leftidx = 0;
    let mut rightidx = 0;
    let mut mainidx = left;
    
    //loop over array range length while left and right are not done
    while left_array.len() > 0 && leftidx < left_array.len() && right_array.len() > 0 && rightidx < right_array.len() {
    //compare left to right and swap as needed
        //if left is less then items current is set to left and increment left
        //if right is less then items current is set to right and increment right
        if right_array.len() > rightidx {
            if left_array.len() > leftidx && left_array[leftidx] <= right_array[rightidx] {
                items[mainidx] = left_array[leftidx];
                leftidx += 1;
                mainidx += 1;
            }
        }

        if left_array.len() > leftidx {
            if right_array.len() > rightidx && right_array[rightidx] < left_array[leftidx] {
                items[mainidx] = right_array[rightidx];
                rightidx += 1;
                mainidx += 1;
            }
        }
    }

    //finally check both left range and right range and add to the end...
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

fn get_array(items: &Vec<u64>, left: usize, mid: usize) -> Vec<u64> {
    items[left..=mid].to_vec()
}