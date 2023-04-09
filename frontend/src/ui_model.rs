pub struct PortalState {
    pub search_text: String,
    chat_groups: Vec<ChatGroup>,
    pub selected_group_idx: Option<usize>,
}

impl PortalState {
    pub fn chat_groups(&self) -> &Vec<ChatGroup> {
        &self.chat_groups
    }
}

pub struct ChatGroup {
    name: String,
    last_message: String,
    #[allow(dead_code)]
    messages: Vec<ChatMessage>,
}

impl ChatGroup {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn last_message(&self) -> &String {
        &self.last_message
    }

    pub fn messages(&self) -> &Vec<ChatMessage> {
        &self.messages
    }
}

pub struct ChatMessage {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    message: String,
}

impl ChatMessage {
    pub fn from(&self) -> &String {
        &self.from
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

fn build_test_messages(group: &str) -> Vec<ChatMessage> {
    (1..100)
        .map(|i| ChatMessage {
            from: format!("{} - name {}", group, i),
            message: format!(
                "message XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX {}",
                i
            ),
        })
        .collect()
}

impl Default for PortalState {
    fn default() -> Self {
        let chat_groups: Vec<_> = (1..100)
            .map(|i| {
                let group = format!("group {}", i);
                let test_messages = build_test_messages(group.as_str());
                ChatGroup {
                    name: group,
                    last_message: format!("last message {}", i),
                    messages: test_messages,
                }
            })
            .collect();

        Self {
            search_text: String::new(),
            chat_groups,
            selected_group_idx: None,
        }
    }
}
