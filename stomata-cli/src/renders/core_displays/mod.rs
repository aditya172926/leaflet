//! Core display modules for system monitoring UI
//!
//! This module contains all the display implementations for different
//! system monitoring views. Each submodule implements the `Display` trait
//! to render specific types of system information in the terminal UI.
//!
//! # Modules
//!
//! - `display_app` - Application-level display and layout
//! - `display_metrics` - System metrics visualization (CPU, memory, disk)
//! - `display_network` - Network interface statistics and connections
//! - `display_processes` - Interactive process list
//! - `display_single_process` - Detailed view of individual processes
//! - `display_system_info` - OS and kernel information display
//! - `traits` - Common display trait definitions

pub mod display_app;
pub mod display_metrics;
pub mod display_network;
pub mod display_processes;
pub mod display_single_process;
pub mod display_system_info;

pub mod traits;
