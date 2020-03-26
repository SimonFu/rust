struct CustomSmartPointer {
    data : String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop CustomSmartPointer object with data: {}", self.data);
    }
}

fn main() {
    let p1 = CustomSmartPointer{data : String::from("Object 1")};
    let p2 = CustomSmartPointer{data : String::from("Object 2")};
    println!("Objects have been created!");
    drop(p2);
    println!("Drop before leave main function");
}
