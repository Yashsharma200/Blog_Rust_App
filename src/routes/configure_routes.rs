use actix_web::web;

use super::routes_services;

pub fn config(config: &mut web::ServiceConfig){
    config
    .service(routes_services::test)
    .service(routes_services::fetch_users)
    .service(routes_services::create_user)
    .service(routes_services::fetch_user_by_id)
    .service(routes_services::create_user_article)
    .service(routes_services::fetch_user_articles);
   
}