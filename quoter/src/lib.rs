#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod builder;

pub use crate::builder::crawler;

pub mod searcher {

    pub fn search(word: String) -> String {

        format!("{} has been searched", word)
    }

}
