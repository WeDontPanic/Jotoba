use actix_session::Session;
use search::query::UserSettings;

// Initializes the session. Returns a session id if user didn't opt out
pub(super) fn init(session: &Session, settings: &UserSettings) -> Option<String> {
    // User opted out
    if !settings.cookies_enabled {
        session.purge();
        return None;
    }

    // Reads or generates a new session id
    let session_id = match session.get::<String>("id").ok()? {
        Some(v) => v,
        None => {
            let new_id = utils::rand_alpha_numeric(30);
            session.set("id", new_id.clone()).ok()?;
            new_id
        }
    };

    Some(session_id)
}
