// Generate the bindings in their own module
pub mod wlr_workspace_unstable {
    use wayland_client;
    // import objects from the core protocol if needed
    use wayland_client::protocol::*;

    // This module hosts a low-level representation of the protocol objects
    // you will not need to interact with it yourself, but the code generated
    // by the generate_client_code! macro will use it
    pub mod __interfaces {
        // import the interfaces from the core protocol if needed
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("./protocols/ext-workspace-unstable-v1.xml");
    }
    use self::__interfaces::*;

    // This macro generates the actual types that represent the wayland objects of
    // your custom protocol
    wayland_scanner::generate_client_code!("./protocols/ext-workspace-unstable-v1.xml");
}