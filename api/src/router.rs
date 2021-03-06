use crate::endpoints::{book, identity, root};
use crate::middleware;

use warp::{filters::BoxedFilter, Filter, Reply};

pub fn router() -> BoxedFilter<(impl Reply,)> {
    // GET - /
    let root = warp::path::end().and(warp::get()).and_then(root::root);

    let identity = warp::path("identity").and(
        warp::path("oauth")
            .and(
                // GET - /identity/oauth/client_identifier
                warp::path("client_identifier")
                    .and(warp::path::end())
                    .and(warp::get())
                    .and_then(identity::get_oauth_client_identifier)
                    // POST - /identity/oauth/authentication
                    .or(warp::path("authentication")
                        .and(warp::path::end())
                        .and(warp::post())
                        .and(warp::body::json())
                        .and_then(identity::create_oauth_authentication)),
            )
            // GET - /identity/session/info
            .or(warp::path!("session" / "info")
                .and(warp::path::end())
                .and(warp::get())
                .and(middleware::session::authorization())
                .and_then(identity::get_session_info)),
    );

    let book = warp::path("book").and(
        // GET /book
        warp::path::end()
            .and(middleware::session::authorization())
            .and(warp::get())
            .and(warp::query())
            .and_then(book::list_books)
            // GET /book/<book_id>
            .or(middleware::session::authorization()
                .and(warp::path::param::<u32>())
                .and(warp::path::end())
                .and(warp::get())
                .and_then(book::get_book_by_id)),
    );

    root.or(identity)
        .or(book)
        .recover(middleware::rejection::handle_rejection)
        .boxed()
}
