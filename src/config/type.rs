use crate::*;

pub type Config = HashMap<String, Vec<Remote>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Remote {
    pub name: String,
    pub url: String,
}
