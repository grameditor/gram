use super::{Client, proto};
use anyhow::{Context as _, Result};
use collections::HashMap;
use gpui::{Context, SharedString, Task};
use postage::watch;
use rpc::proto::{RequestMessage, UsersResponse};
use std::sync::{Arc, Weak};

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
    client: Weak<Client>,
}

impl UserStore {
    pub fn new(client: Arc<Client>, _cx: &Context<Self>) -> Self {
        let (_current_user_tx, current_user_rx) = watch::channel();

        Self {
            users: Default::default(),
            by_email: Default::default(),
            current_user: current_user_rx,
            client: Arc::downgrade(&client),
        }
    }

    #[cfg(feature = "test-support")]
    pub fn clear_cache(&mut self) {
        self.users.clear();
        self.by_email.clear();
    }

    pub fn get_users(
        &self,
        user_ids: Vec<u64>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let mut user_ids_to_fetch = user_ids.clone();
        user_ids_to_fetch.retain(|id| !self.users.contains_key(id));

        cx.spawn(async move |this, cx| {
            if !user_ids_to_fetch.is_empty() {
                this.update(cx, |this, cx| {
                    this.load_users(
                        proto::GetUsers {
                            user_ids: user_ids_to_fetch,
                        },
                        cx,
                    )
                })?
                .await?;
            }

            this.read_with(cx, |this, _| {
                user_ids
                    .iter()
                    .map(|user_id| {
                        this.users
                            .get(user_id)
                            .cloned()
                            .with_context(|| format!("user {user_id} not found"))
                    })
                    .collect()
            })?
        })
    }

    pub fn get_user(&self, user_id: u64, cx: &Context<Self>) -> Task<Result<Arc<User>>> {
        if let Some(user) = self.users.get(&user_id).cloned() {
            return Task::ready(Ok(user));
        }

        let load_users = self.get_users(vec![user_id], cx);
        cx.spawn(async move |this, cx| {
            load_users.await?;
            this.read_with(cx, |this, _| {
                this.users
                    .get(&user_id)
                    .cloned()
                    .context("server responded with no users")
            })?
        })
    }

    pub fn current_user(&self) -> Option<Arc<User>> {
        self.current_user.borrow().clone()
    }

    pub fn watch_current_user(&self) -> watch::Receiver<Option<Arc<User>>> {
        self.current_user.clone()
    }

    fn load_users(
        &self,
        request: impl RequestMessage<Response = UsersResponse>,
        cx: &Context<Self>,
    ) -> Task<Result<Vec<Arc<User>>>> {
        let client = self.client.clone();
        cx.spawn(async move |this, cx| {
            if let Some(rpc) = client.upgrade() {
                let response = rpc.request(request).await.context("error loading users")?;
                let users = response.users;

                this.update(cx, |this, _| this.insert(users))
            } else {
                Ok(Vec::new())
            }
        })
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
