fn main() {
    let mut x = 5;
    { // Limit the scope of the mutable borrow.
        let y = &mut x; 
        *y += 1; 
    }
    { // Limit the scope of the mutable borrow.
        let z = &mut x; 
        *z += 1; 
    }
    println!("x = {}", x);
}