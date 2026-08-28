use std::sync::Arc;

use crate::core::workers::{BoxManager, EphemeralBox, PersistentBox};

pub struct AppState {
    pub executor_pool: Arc<BoxManager<EphemeralBox>>,
    pub compiler_pool: Arc<BoxManager<PersistentBox>>
}

impl AppState {
    pub fn new(ephemeral_worker_count: usize, persistent_worker_count: usize) -> Self {

        AppState {
            executor_pool: Arc::new(BoxManager::new(0..ephemeral_worker_count, |id| EphemeralBox::new(id))),
            compiler_pool: Arc::new(
                    BoxManager::new(0.. persistent_worker_count, |id| PersistentBox::new(id)
                )
            ),
        }
    }
}
