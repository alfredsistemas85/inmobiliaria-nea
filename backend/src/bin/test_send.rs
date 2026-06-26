use backend::api::dashboard::controllers::get_stats;

fn assert_send<T: Send>(_: T) {}

fn main() {
    let _ = assert_send(get_stats);
}
