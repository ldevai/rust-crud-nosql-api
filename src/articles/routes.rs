use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{auth, environment};
use crate::auth::models::Role;
use crate::environment::Environment;
use crate::articles::handlers;

pub fn routes(_env: Environment) -> BoxedFilter<(impl Reply, )> {
    let get_home_articles_route = warp::get().and(warp::path!("api" / "articles_home")
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_home_articles_handler));

    let get_articles_route = warp::get().and(warp::path!("api" / "articles")
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_articles_handler));

    let get_article_route = warp::get().and(warp::path!("api" / "articles" / String)
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::get_article_by_url_handler));

    let create_article_route = warp::post().and(warp::path!("api" / "articles")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::create_article_handler));

    let update_article_route = warp::put().and(warp::path!("api" / "articles")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::update_article_handler));

    let delete_article_route = warp::delete().and(warp::path!("api" / "articles" / String)
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::delete_article_handler));

    let update_home_view_route = warp::get().and(warp::path!("api" / "articles" / "updateHomeView" / String)
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::update_home_view_handler));

    let delete_comment_route = warp::delete().and(warp::path!("api" / "articles" / "comments" / String / String)
        .and(environment::with_env(_env.clone()))
        .and(auth::middleware::with_auth(Role::Admin))
        .and_then(handlers::delete_comment_handler));

    let post_comment_route = warp::post().and(warp::path!("api" / "articles" / "comments")
        .and(warp::body::json())
        .and(environment::with_env(_env.clone()))
        .and_then(handlers::post_comment_handler));

    let routes = get_home_articles_route
        .or(get_articles_route)
        .or(get_article_route)
        .or(create_article_route)
        .or(update_article_route)
        .or(delete_article_route)
        .or(update_home_view_route)
        .or(delete_comment_route)
        .or(post_comment_route);

    routes.boxed()
}
