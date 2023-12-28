struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }

    fn can_hold(&self, other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    
}

fn main() {
    let rect=Rectangle{
        width:56,
        height:78,
    };

    let rect2=Rectangle{
        width:34,
        height:253,
    };
    let rect3=Rectangle{
        width:57,
        height:69,
    };
    print!("The area of the rectangle is {} square pixels.",rect.area());
    print!("rect can hold {}",rect.can_hold(&rect2));
    print!("rect can hold {}",rect.can_hold(&rect3));

}