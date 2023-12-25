use crate::util;
pub mod server;
pub mod connection;

pub struct bsp_opts {
    send_threads : u8,
    send_assist_threads : u8,
    recv_threads : u8,
    recv_assist_threads : u8
}

fn bsp<T, MG, MC, SG, SL>(
    signal : SG,
    slot : SL,
    before_signal : fn(),
    after_signal : fn(),
    opts : bsp_opts
) 
where
    MG : Fn(i32, T),
    SG : Fn(MG),
    MC : Fn(T),
    SL : Fn(MC),
{
    for i in 0..opts.send_threads {
        // how to interact with rsmpi?
    }
}