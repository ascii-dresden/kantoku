use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Member {
    pub first: String,
    pub last: String,
    // TODO: Is there a good mail address crate?
    pub mail: String,
    pub tg_handle: Option<String>,
}

type Memberlist = Vec<Member>;
