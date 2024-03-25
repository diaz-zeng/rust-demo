pub mod message;

#[derive(Debug)]
pub struct Entity {
    name: String,
}
impl Entity {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }
}
