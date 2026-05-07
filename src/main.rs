use std::io::{self, Read};

use wayland_client::protocol::{wl_registry, wl_seat};
use wayland_client::{Connection, Dispatch, EventQueue, QueueHandle, delegate_noop};

use crate::other_protocols::{zwp_input_method_manager_v2, zwp_input_method_v2};

mod other_protocols;

fn main() {
    let conn = Connection::connect_to_env().expect("no compositor running?");
    let mut event_queue = conn.new_event_queue();

    // get input from stdin
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    // create the state with the required objects
    let mut state = construct_state(&conn, &mut event_queue, buffer)
        .expect("compositor doesn't support the `input_method_unstable_v2` protocol");

    // get an input method
    state
        .input_method_manager
        .get_input_method(&state.seat, &event_queue.handle(), ());

    // get the input method
    event_queue.roundtrip(&mut state).unwrap();

    // process the events from the input method
    event_queue.roundtrip(&mut state).unwrap();

    // cleanup
    event_queue.roundtrip(&mut state).unwrap();
}

fn construct_state(
    conn: &Connection,
    state_event_queue: &mut EventQueue<State>,
    text: String,
) -> Option<State> {
    let mut event_queue = conn.new_event_queue();

    // send `get_registry` request
    {
        let queue_handle = event_queue.handle();
        let state_queue_handle = state_event_queue.handle();
        let display = conn.display();
        display.get_registry(&queue_handle, state_queue_handle);
    }

    let mut constructor = StateConstructor::default();
    event_queue.roundtrip(&mut constructor).unwrap();

    constructor.construct(text)
}

struct State {
    seat: wl_seat::WlSeat,
    input_method_manager: zwp_input_method_manager_v2::ZwpInputMethodManagerV2,
    text: String,
}

impl Dispatch<zwp_input_method_v2::ZwpInputMethodV2, ()> for State {
    fn event(
        state: &mut Self,
        proxy: &zwp_input_method_v2::ZwpInputMethodV2,
        event: <zwp_input_method_v2::ZwpInputMethodV2 as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        let zwp_input_method_v2::Event::Done = event else {
            return;
        };

        // commit the string only after the `Event::Done` is received
        proxy.commit_string(std::mem::take(&mut state.text));
        proxy.commit(0);
    }
}

delegate_noop!(State: ignore wl_seat::WlSeat);
delegate_noop!(State: zwp_input_method_manager_v2::ZwpInputMethodManagerV2);

#[derive(Default)]
struct StateConstructor {
    seat: Option<wl_seat::WlSeat>,
    input_method_manager: Option<zwp_input_method_manager_v2::ZwpInputMethodManagerV2>,
}

impl StateConstructor {
    pub fn construct(self, text: String) -> Option<State> {
        let seat = self.seat?;
        let input_method_manager = self.input_method_manager?;
        Some(State {
            seat,
            input_method_manager,
            text,
        })
    }
}

impl Dispatch<wl_registry::WlRegistry, QueueHandle<State>> for StateConstructor {
    fn event(
        state: &mut Self,
        proxy: &wl_registry::WlRegistry,
        event: <wl_registry::WlRegistry as wayland_client::Proxy>::Event,
        state_qhandle: &QueueHandle<State>,
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match interface.as_str() {
                "wl_seat" => {
                    state.seat = Some(proxy.bind(name, version, state_qhandle, ()));
                }

                "zwp_input_method_manager_v2" => {
                    state.input_method_manager = Some(proxy.bind(name, version, state_qhandle, ()))
                }

                _ => {}
            }
        }
    }
}
