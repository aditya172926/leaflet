//! Core application features
//!
//! This module provides the main interactive system monitoring and utility
//! functionality of the application. The core feature runs exclusively in
//! TUI (Terminal User Interface) mode and offers real-time system metrics,
//! process monitoring, and various system utilities.
//!
//! # Features
//!
//! - Real-time system resource monitoring (CPU, memory, disk, network)
//! - Interactive process management
//! - System utility tools
//! - Optional metrics data storage for historical analysis
//!
//! # Usage
//!
//! The core feature is accessed through the interactive TUI:
//!
//! ```bash
//! # Launch with default 1-second refresh rate
//! stomata -i
//!
//! # Launch with faster refresh (500ms)
//! stomata -i --interval 500
//!
//!
//! # Modules
//!
//! - [`core_feature`] - Main entry point and render loop implementation

pub mod core_feature;