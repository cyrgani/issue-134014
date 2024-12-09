use enginelib::{EngineAPI, Registry};

#[no_mangle]
pub fn run(api: &mut EngineAPI) {
    api.register();
}
