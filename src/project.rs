pub mod project {
    use crate::entry::entry::Entry;
    use chrono::{DateTime, Utc, Local};

    pub struct Project {
        name: String,
        description: Option<String>,
        start: DateTime<Local>,
        end: Option<DateTime<Local>>,
        is_active: bool,
        entries: Vec<Entry>
    }


    impl Project {
        pub fn new(name: &str, description: Option<&str>, start: DateTime<Local>, end: Option<DateTime<Local>>, is_active: bool) -> Self {
            let entries: Vec<Entry> = vec!();
            Project {
                name: String::from(name),
                description: match description {
                    Some(value) => Some(String::from(value)),
                    None => None
                },
                start,
                end,
                is_active,
                entries }
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