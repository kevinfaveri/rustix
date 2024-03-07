pub mod cat;
pub mod user;

use crate::Error;

pub async fn sync_indexes() -> Result<(), Error> {
    Ok(())
}
