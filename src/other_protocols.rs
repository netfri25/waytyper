
// disabling the warning since the `generate_client_code!` macro depends on `wayland_client`
// existing in `super` (i.e. `super::wayland_client`)
#[allow(clippy::single_component_path_imports)]
use wayland_client;
use wayland_client::protocol::*;

pub mod __interfaces {
    use wayland_client::protocol::__interfaces::*;
    wayland_scanner::generate_interfaces!("protocols/input-method-unstable-v2.xml");
}
use self::__interfaces::*;

use wayland_protocols::wp::text_input::zv3::client::zwp_text_input_v3;
wayland_scanner::generate_client_code!("protocols/input-method-unstable-v2.xml");
