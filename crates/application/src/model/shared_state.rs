use domain::factory::shared_state::SharedStateFactory;

#[allow(dead_code)]
pub struct SharedStateUseCase<T: SharedStateFactory> {
    pub shared_state_factory: T,
}

impl<T: SharedStateFactory> SharedStateUseCase<T> {
    pub async fn new(shared_state_factory: T) -> Self {
        Self {
            shared_state_factory,
        }
    }
}
