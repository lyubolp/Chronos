pub mod project {
    use crate::entry::entry::Entry;
    use chrono::{DateTime, Utc};

    pub struct Project {
        name: String,
        description: Option<String>,
        start: DateTime<Utc>,
        end: Option<DateTime<Utc>>,
        is_active: bool,
        entries: Vec<Entry>
    }


    impl Project {
        pub fn new(name: String, description: Option<String>, start: DateTime<Utc>, end: Option<DateTime<Utc>>, is_active: bool) -> Self {
            let entries: Vec<Entry> = vec!();
            Project { name, description, start, end, is_active, entries }
        }

        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn description(&self) -> &Option<String> {
            &self.description
        }
        pub fn start(&self) -> DateTime<Utc> {
            self.start
        }
        pub fn end(&self) -> Option<DateTime<Utc>> {
            self.end
        }
        pub fn is_active(&self) -> bool {
            self.is_active
        }
        pub fn entries(&self) -> &Vec<Entry> {
            &self.entries
        }

        pub fn rename(&mut self, new_name: &str) {
            self.name = String::from(new_name);
        }
        pub fn add_entry(&mut self, entry: Entry) {
            self.entries.push(entry);
        }

        pub fn remove_entry(&mut self, id: u32) {
            let mut index: Option<usize> = None;

            for (i, entry) in self.entries.iter().enumerate() {
                if entry.get_id() == id {
                    index = Some(i);
                }
            }

            match index {
                Some(index) => {
                    self.entries.remove(index);
                },
                None => println!("Can't find index")
            };
        }

    }
}