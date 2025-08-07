use std::{any::TypeId, thread::sleep, time::Duration};

use typed_log::{Loggable, log_any};
use typed_log_rerun::IncrementTime;

pub struct MyCustomStruct {
    pub position: [f32; 3],
}

impl Loggable for MyCustomStruct {
    fn type_id(&self) -> std::any::TypeId {
        TypeId::of::<Self>()
    }
}

fn logger(my_struct: &MyCustomStruct) {
    typed_log_rerun::get_rr()
        .log(
            format!("position {:p}", &my_struct),
            &rerun::Points3D::new(&[my_struct.position]),
        )
        .unwrap();
}
fn main() {
    typed_log::push_log_impl(&logger);
    typed_log_rerun::register_typed_loggers();
    typed_log_rerun::init_rr();

    for i in 0..10 {
        log_any(&IncrementTime(1));
        log_any(&MyCustomStruct {
            position: [0f32, 0f32, i as f32 / 10f32],
        });
    }
    // TODO: find a way to wait for rerun streaming to end.
    sleep(Duration::from_secs_f32(1f32));
}
