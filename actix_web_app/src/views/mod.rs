mod auth;
use auth::auth_view_factory;
use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig)
{
    auth_view_factory(app);
}
