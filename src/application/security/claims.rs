use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate)  struct Claims {
    pub(crate)  sub: String,
    pub(crate)  exp: usize,
}
