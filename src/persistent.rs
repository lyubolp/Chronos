pub mod persistent {
    trait Persistent {
        fn save(&self) -> str;
        fn load(content: &str) -> Self;
    }
}