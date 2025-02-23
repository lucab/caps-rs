use caps::{runtime, CapsBitFlags};

fn main() {
    let amb_set = runtime::ambient_set_supported().is_ok();
    println!("Ambient set supported: {}", amb_set);

    let all: CapsBitFlags = caps::all_caps();
    let supported = runtime::thread_all_supported_caps();
    let missing = all.difference(supported);
    println!("Unsupported new capabilities: {:?}", missing);
}
