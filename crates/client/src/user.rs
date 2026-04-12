use super::{Client, proto};
use collections::HashMap;
use gpui::{Context, SharedString};
use postage::watch;
use std::sync::Arc;

pub type UserId = u64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ProjectId(pub u64);

impl ProjectId {
    pub fn to_proto(self) -> u64 {
        self.0
    }
}

#[derive(Default, Debug)]
pub struct User {
    pub id: UserId,
    pub email: SharedString,
    pub name: Option<String>,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.email.cmp(&other.email)
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.email == other.email
    }
}

impl Eq for User {}

pub struct UserStore {
    users: HashMap<u64, Arc<User>>,
    by_email: HashMap<SharedString, u64>,
    current_user: watch::Receiver<Option<Arc<User>>>,
}

impl UserStore {
    pub fn new(_client: Arc<Client>, _cx: &Context<Self>) -> Self {
        let (_current_user_tx, current_user_rx) = watch::channel();

        Self {
            users: Default::default(),
            by_email: Default::default(),
            current_user: current_user_rx,
        }
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        self.current_user.borrow().clone()
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    pub fn insert(&mut self, users: Vec<proto::User>) -> Vec<Arc<User>> {
        let mut ret = Vec::with_capacity(users.len());
        for user in users {
            let user = User::new(user);
            if let Some(old) = self.users.insert(user.id, user.clone())
                && old.email != user.email
            {
                self.by_email.remove(&old.email);
            }
            self.by_email.insert(user.email.clone(), user.id);
            ret.push(user)
        }
        ret
    }
}

impl User {
    fn new(message: proto::User) -> Arc<Self> {
        Arc::new(User {
            id: message.id,
            email: message.email.into(),
            name: message.name,
        })
    }
}
