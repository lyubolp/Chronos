pub mod entry{
    use chrono::{DateTime, Utc};

    pub struct Entry {
        id: u32,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        comment: Option<String>
    }

    impl Entry {
        pub fn new(start: DateTime<Utc>, end: Option<DateTime<Utc>>, comment: Option<String>) -> Self {
            Entry {
                id: generate_entry_id(),
                start,
                end,
                comment
            }
        }

        pub fn from_string(source: &str) -> Self {
            unimplemented!("Not yet implemented !")
        }

        pub fn get_id(&self) -> u32 {
            self.id
        }
        pub fn get_start(&self) -> DateTime<Utc> {
            self.start
        }
        pub fn get_end(&self) -> Option<DateTime<Utc>> {
            self.end
        }
        pub fn get_comment(&self) -> Option<String> {
            match &self.comment {
                Some(comment) => Some(comment.clone()),
                None => None
            }
        }

        pub fn to_string(&self) -> String {
            unimplemented!("Not yet implemented !")
        }
    }

    fn generate_entry_id() -> u32 {
        unimplemented!("Not yet implemented !")
    }

}