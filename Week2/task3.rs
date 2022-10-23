// ====================================
// BÀI 3:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


fn main() {
    iter_num(12345);
}

pub fn iter_num(num: i32) -> bool {

    let num_str = num.to_string();

    let num_str_cl = num_str.clone();
    let chars = num_str_cl.chars(); // <-- move occurs because `chars` has type `Chars<'_>`, which does not implement the `Copy` trait
    // => creating a new copy of this object by add .clone() to solve this problem

    let cahrs_cl = chars.clone();
    let len = cahrs_cl.count();     // <-- `chars` moved due to this method call
    // => creating a new copy of this object by add .clone() to solve this problem

    println!("Len = {:?}", len);

    for c in chars {             // <-- ❌ "value used here after move": chars
        println!(">>> {:?}", c);
    }

    return true;
}

/**
Result:
Len = 5
>>> '1'
>>> '2'
>>> '3'
>>> '4'
>>> '5'
 */