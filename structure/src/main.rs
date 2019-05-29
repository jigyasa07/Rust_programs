struct rectangle{
		width:u32,
		height:u32,
	}

fn main() {
    let rect= rectangle { width:10, height:40};
    println!("area of rectangle is {}",area(&rect));
}

fn area(Rectangle:&rectangle)->u32{
	Rectangle.width*Rectangle.height
}
