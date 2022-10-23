// ====================================
// BÀI 4:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


// First solution: Create new copy version of object needed to use again by using .clone()
fn main() {
    let mut v = vec![1, 2, 3];

    let mut v_cl = v.clone();

    go(&mut v_cl);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v_cl.len())
}

fn go(v: &mut Vec<i32>) {
    for i in v.iter() {
        println!("{}", i);
    }
    v.push(4);
}




// Second solution: Change the go function to return new Vec object and assign it to v variable.
fn main() {
    let mut v = vec![1, 2, 3];

    v = go(&mut v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

fn go(v: &mut Vec<i32>) -> Vec<i32> {
    for i in v.iter() {
        println!("{}", i);
    }
    v.push(4);

    return v.to_vec();
}
