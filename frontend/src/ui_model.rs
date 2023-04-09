
pub struct MyApp {
    pub search_text: String,
    chat_groups: Vec<ChatGroup>
}

impl MyApp {
    pub fn chat_groups(&self) -> &Vec<ChatGroup> {
        &self.chat_groups
    }
}

pub struct ChatGroup {
    name: String,
    last_message: String,
}

impl ChatGroup {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn last_message(&self) -> &String {
        &self.last_message
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let chat_groups: Vec<_> = (1..100).map(|i| {
            ChatGroup {
                name: format!("name {}", i),
                last_message: format!("message {}", i), // "message 1".to_string(),
            }
        }).collect();

        Self {
            search_text: String::new(),
            chat_groups
        }
    }
}
