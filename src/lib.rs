//! This crate provides bindings to the wlroots wayland protocol extensions
//! provided in <https://gitlab.freedesktop.org/wlroots/wlr-protocols>
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.

#![warn(missing_docs)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

pub mod ext_workspace_unstable {
    //! Protocol for general workspaces (used specifically for Hyprland)

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!("./protocols/ext-workspace-unstable-v1.xml",
            []
        );
    }
}