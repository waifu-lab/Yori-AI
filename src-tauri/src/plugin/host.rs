wit_bindgen::generate!({
    world: "context-impl",
    path: "wit/plugin.wit"
});

use exports::waifu_lab::yori_ai::context::{GuestHost, EventData};

struct HostImpl;

impl GuestHost for HostImpl {
    #[allow(async_fn_in_trait)]
    fn on_event(&self, event: EventData) -> () {
        todo!()
    }

    #[allow(async_fn_in_trait)]
    fn search_memory(&self, query: String, limit: u32) -> Vec::<String> {
        todo!()
    }
}