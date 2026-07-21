use axum::Router;
use crate::{
    app_state::AppState,
    routes::{
        auth_routes,
        game_genres_routes,
        game_platforms_routes,
        game_routes,
        genres_routes,
        library_routes,
        platform_routes,
        refresh_token_routes,
        review_routes,
        user_routes
    },
};

pub fn create_router(state:AppState) -> Router {
        Router::new()
        .merge(auth_routes())
        .merge(game_genres_routes())
        .merge(game_platform_routes())
        .merge(game_routes())
        .merge(genres_routes())
        .merge(library_routes())
        .merge(platform_routes())
        .merge(refresh_token_routes())
        .merge(review_routes())
        .merge(user_routes())
        .with_state(state);

        Ok(Router::new())
    }
