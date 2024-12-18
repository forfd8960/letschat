use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::Ok;

use crate::{
    errors::ChatError,
    pb::chat::{ChatGroup, User},
};

#[derive(Debug, Clone)]
pub struct ChatState {
    pub inner: Arc<Mutex<ChatStateInner>>,
}

#[derive(Debug, Clone)]
pub struct ChatStateInner {
    pub user_store: UserList,
    pub chat_groups: ChatGroups,
}

#[derive(Debug, Clone)]
pub struct UserList {
    pub users: HashMap<i64, User>,
}

#[derive(Debug, Clone)]
pub struct ChatGroups {
    pub groups: HashMap<i64, ChatGroup>,
}

impl ChatGroups {
    pub fn add_group(&mut self, group: ChatGroup) {
        self.groups.insert(group.id, group)
    }

    pub fn get_group(&self, id: i64) -> Option<ChatGroup> {
        self.groups.get(&id)
    }

    pub fn add_member_to_group(&mut self, id: i64, member: User) -> Result<(), ChatError> {
        let group = self.get_group(id);
        if group.is_none() {
            return Err(ChatError::GroupNotFound(format!("{}", id)));
        }

        let g = group.unwrap();
        g.members.push(user);
        Ok(())
    }
}
