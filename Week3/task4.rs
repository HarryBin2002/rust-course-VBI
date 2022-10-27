///////////////////////////////////////////
// BAI 4
// Yêu cầu :
// + Sửa lại code sao cho compile cho đúng 
///////////////////////////////////////////


#[derive(Debug, Clone)]
struct MyData {
    val1: i32,
    val2: String,
}



fn main() {
    let d = MyData {
        val1: 35,
        val2: String::from("Hello World"),
    };

    let both = d.clone().get_both();
    let x = d.clone().get_val1();
    let y = d.clone().get_val2();

    // println!("both: {:?}, val1: {}, val2: {}", both, x, y);
}


impl MyData {
    pub fn get_val1(self) -> i32 {
        return self.val1.clone();
    }

    pub fn get_val2(self) -> String {
        return self.val2.clone();
    }

    pub fn get_both(self) -> (i32, String) {
        return (self.val1, self.val2);
    }
}