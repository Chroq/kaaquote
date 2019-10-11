pub mod builder;

pub use crate::builder::crawler;

pub mod searcher {

    pub fn search(word: String) -> String {

        format!("{} has been searched", word)
    }

}
