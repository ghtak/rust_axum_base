mod extract;
mod route;

pub use route::session_route;

pub(crate) const SESSIONID: &'static str = "SESSIONID";

pub(crate) type StoreImpl = async_session::MemoryStore;
