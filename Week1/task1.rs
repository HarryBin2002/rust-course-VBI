fn main() {
    // Example:
    // let org_arr = [1, 2, 3, 5, 6, 8, 110, 11, 6, 8, 10, 13];
    // let sub_arr = [6, 86, 10];

    // Call function scanning_arr here to run programing.
    scanning_arr(&org_arr, &sub_arr);
}

/**
 * This function to excute the logic code: 
 * loop all elements from original list if met the element is equal to the first element of sub list that are stopped.
 * Create a new list (Vec type is used here) and push new elements sequence so that
 * Compare between new list (Vec) and sub list before.
 * If they are the same, return true, else rteturn false;
 */
pub fn scanning_arr(org_arr: &[u32], sub_arr: &[u32]) {
    let mut checking = false;
    
    for i in 0..org_arr.len() {
        if org_arr[i] == sub_arr[0] {
            let mut v = Vec::new();
            let mut k = i;
            for _j in 0..sub_arr.len() {
                v.push(org_arr[k]);
                k += 1;
            }
    
            if comparison_arr(&v, &sub_arr) == true {
                checking = true;
                break;
            }
        }
    }
    
    if checking == true {
        println!("result: {}", "YES")
    } else {
        println!("result: {}", "NO");
    }
}

// This function to compare 2 list of number
// The first list is the slice is cut out from original list
// The second list is the sub-list
pub fn comparison_arr(sl_arr: &[u32], sub_arr: &[u32]) -> bool {
    return sl_arr == sub_arr;
} 