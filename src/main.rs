
fn main() {

    let mut x:Option<u32> = None;
    let y = x.take();
    println!("{:?}", x);
    println!("{:?}", y);

}


