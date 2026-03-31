// This library shares code with the shoes binary. Server-side code appears "unused"
// in lib builds but is used by: (1) the binary for server pub mode, (2) FFI for mobile.
// The client/server code is intermingled within pub modules - a proper fix would require
// splitting into separate client/server pub modules or using feature flags.
#![allow(dead_code)]

//! shoes - A high-performance multi-protocol proxy server.
//!
//! This library provides the core functionality for shoes, enabling it to be
//! embedded in mobile applications (Android/iOS) as a VPN backend.
//!
//! # Features
//!
//! - **Multi-protocol support**: VLESS, VMess, Trojan, Shadowsocks, and more
//! - **TUN device support**: Virtual network interface for VPN pub mode
//! - **Proxy chaining**: Connect through multiple proxies
//! - **Flexible routing**: Rule-based traffic routing
//!
//! # Mobile Integration
//!
//! For Android, use the FFI pub module:
//!
//! ```kotlin
//! // Load native library
//! System.loadLibrary("shoes")
//!
//! // Initialize
//! ShoesNative.init("info")
//!
//! // Start VPN with TUN fd from VpnService
//! val handle = ShoesNative.startTun(tunFd, configYaml, protectCallback)
//!
//! // Stop VPN
//! ShoesNative.stop(handle)
//! ```
//!
//! For iOS, use the C FFI pub module from Swift:
//!
//! ```swift
//! // Initialize
//! shoes_init("info")
//!
//! // Start VPN with packet tunnel fd
//! let handle = shoes_start(configYaml, protectCallback)
//!
//! // Stop VPN
//! shoes_stop(handle)
//! ```
//!
//! # Platform Support
//!
//! - Linux (x86_64, aarch64)
//! - Android (arm64-v8a, armeabi-v7a, x86_64)
//! - iOS (arm64)

// pub modules are declared here (mirroring main.rs) so the library crate can
// expose them for FFI/mobile integration.
pub mod address;
pub mod anytls;
pub mod async_stream;
pub mod buf_reader;
pub mod client_proxy_chain;
pub mod client_proxy_selector;
pub mod copy_bidirectional;
pub mod copy_bidirectional_message;
pub mod crypto;
pub mod dns;
pub mod h2mux;
pub mod http_handler;
pub mod hysteria2_server;
pub mod mixed_handler;
pub mod naiveproxy;
pub mod option_util;
pub mod port_forward_handler;
pub mod quic_server;
pub mod quic_stream;
pub mod reality;
pub mod reality_client_handler;
pub mod resolver;
pub mod routing;
pub mod rustls_config_util;
pub mod rustls_connection_util;
pub mod shadow_tls;
pub mod shadowsocks;
pub mod slide_buffer;
pub mod snell;
pub mod socket_util;
pub mod socks5_udp_relay;
pub mod socks_handler;
pub mod stream_reader;
pub mod sync_adapter;
pub mod tcp;
pub mod thread_util;
pub mod tls_client_handler;
pub mod tls_server_handler;
pub mod trojan_handler;
pub mod tuic_server;
pub mod uot;
pub mod util;
pub mod uuid_util;
pub mod vless;
pub mod vmess;
pub mod websocket;
pub mod xudp;

/// Configuration types.
pub mod config;

/// Multi-output logging infrastructure.
pub mod logging;

/// TUN device support for VPN pub mode.
#[cfg(unix)]
pub mod tun;

/// FFI bindings for mobile platforms.
#[cfg(any(target_os = "android", target_os = "ios", feature = "ffi"))]
pub mod ffi;
