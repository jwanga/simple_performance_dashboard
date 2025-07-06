# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Windows performance monitoring dashboard written in Rust targeting Windows x86_64 and ARM64 architectures. The application monitors various system performance metrics including CPU, GPU, memory, and temperature sensors, displaying real-time values and historical graphs.

## Development Commands

### Build and Run
- `cargo build --target x86_64-pc-windows-msvc` - Build for Windows x86_64
- `cargo build --target aarch64-pc-windows-msvc` - Build for Windows ARM64
- `cargo build --release --target x86_64-pc-windows-msvc` - Release build for Windows x86_64
- `cargo build --release --target aarch64-pc-windows-msvc` - Release build for Windows ARM64

#### ARM64 Build Issues on Windows
If you encounter "The parameter is incorrect. (os error 87)" when building for ARM64:

1. **Clean and retry**:
   ```bash
   cargo clean
   cargo build --target aarch64-pc-windows-msvc
   ```

2. **Use shorter path**: Move project to a shorter path (e.g., `C:\temp\dashboard`)

3. **Set environment variable**:
   ```bash
   set CARGO_TARGET_DIR=C:\temp\cargo-target
   cargo build --target aarch64-pc-windows-msvc
   ```

4. **Install ARM64 target** (if not already installed):
   ```bash
   rustup target add aarch64-pc-windows-msvc
   ```

5. **Alternative: Use cross-compilation tool**:
   ```bash
   cargo install cross
   cross build --target aarch64-pc-windows-msvc
   ```

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test

### Documentation
- `cargo doc` - Generate documentation
- `cargo doc --open` - Generate and open documentation

### Maintenance
- `cargo clean` - Remove build artifacts
- `cargo update` - Update dependencies

## Architecture

The project follows a separation of concerns between presentation and logic layers as specified in requirements.md. Key architectural considerations:

### Performance Requirements
- **Compute Efficiency**: Code must be highly optimized to avoid skewing performance measurements
- **Memory Efficiency**: Minimal memory footprint to prevent interference with system monitoring

### Monitoring Capabilities
The application tracks:
- CPU: utilization %, speed (MHz), temperature, throttling status
- GPU: speed (MHz), temperature, throttling status  
- Memory: utilization %, speed (MHz)
- Temperatures: CPU package/hotspot, GPU package/hotspot, chipset, chassis, additional sensors
- Fans: RPM measurements

### Data Display
For each metric, the application provides:
- Current scalar value
- Session maximum/minimum values
- Historical time-series graphs

## Project Structure

- `src/main.rs` - Main application entry point (currently minimal)
- `Cargo.toml` - Project configuration and dependencies
- `REQUIREMENTS.md` - Detailed functional and technical requirements

## Current Status

The project is in early development with basic Rust project structure in place. The main application logic for performance monitoring is yet to be implemented.