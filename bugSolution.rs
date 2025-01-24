use std::cell::RefCell;

fn main() {    let x = RefCell::new(5);    let y = x.borrow_mut();    let z = x.borrow_mut(); //this will cause a panic at runtime. The code below handles this
    //*y = 6; //This line is commented out since this will cause a panic
    //*z = 7; //This line is also commented out since this will cause a panic

    // Proper solution using RefCell to manage interior mutability
    let mut x = RefCell::new(5);
    {
        let mut y = x.borrow_mut();
        *y = 6;
    }
    {
        let mut z = x.borrow_mut();
        *z = 7;
    }
    println!("x = {}", x.borrow());
} 