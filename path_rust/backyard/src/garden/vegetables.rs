pub struct Asparagus {
    pub name: String,
}

impl Asparagus {
    pub fn echo_name(&self) {
        println!("my name is {}" , self.name);
    }
}