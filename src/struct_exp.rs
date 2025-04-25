pub struct User{
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64
}

pub struct React{
    pub width: u32,
    pub height:u32
}

impl React {
    pub fn area(&self)->u32{
        self.width * self.height
    }
}