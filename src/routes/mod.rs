mod payment_event;
mod user_payment_state;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.configure(payment_event::configure)
        .configure(user_payment_state::configure);
}
