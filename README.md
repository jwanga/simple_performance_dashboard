# Simple Performance Dashboard

A lightweight, real-time desktop application that monitors key hardware performance metrics on Windows systems. Built with Rust and egui for optimal performance with minimal system impact.

## Features

- **Real-time Hardware Monitoring**: Track CPU, GPU, memory, storage, and motherboard metrics
- **Historical Data**: View current, minimum, maximum values and time-series graphs for each metric
- **Efficient Design**: <2% CPU usage, <100MB memory footprint
- **Cross-Architecture Support**: Works on Windows x86_64 and ARM64
- **Responsive UI**: Collapsible sections with data-aware expansion states

### Monitored Metrics

- **CPU**: Utilization, clock speed, voltage, power, temperatures, throttling status
- **GPU**: Utilization, clock speed, memory, voltage, power, temperatures, throttling status  
- **Memory**: Utilization, clock speed, temperature
- **Storage**: Read/write speeds, temperature
- **Motherboard**: Chipset/chassis temperatures, fan speeds, AIO pump speed

## Installation

### Prerequisites

- Windows 10/11 (x86_64 or ARM64)
- Rust toolchain (latest stable)

### Install Rust

If you don't have Rust installed:

```bash
# Install via rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or download from https://rustup.rs/
```

### Install Windows Targets

```bash
# For x86_64 (most common)
rustup target add x86_64-pc-windows-msvc

# For ARM64 (Surface Pro X, etc.)
rustup target add aarch64-pc-windows-msvc
```

### Download and Build

```bash
# Clone the repository
git clone <repository-url>
cd simple_performance_dashboard

# Build and run (x86_64)
cargo run

# Or build for specific target
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
```

## Usage

1. **Launch the application**:
   ```bash
   cargo run
   ```

2. **Navigate the interface**:
   - Click section headers to expand/collapse metric groups
   - Hover over graphs to see crosshair with precise values
   - Sections with available data expand automatically
   - Sections without data show "(No Data)" and remain collapsed

3. **Stop monitoring**: Close the application window or press Ctrl+C in terminal

## Troubleshooting

### ARM64 Build Issues

If you encounter "The parameter is incorrect. (os error 87)" when building for ARM64:

```bash
# Try these solutions in order:

# 1. Clean and retry
cargo clean
cargo build --target aarch64-pc-windows-msvc

# 2. Use shorter path
# Move project to C:\temp\dashboard or similar

# 3. Set custom target directory
set CARGO_TARGET_DIR=C:\temp\cargo-target
cargo build --target aarch64-pc-windows-msvc

# 4. Use cross-compilation (alternative)
cargo install cross
cross build --target aarch64-pc-windows-msvc
```

### Sensor Data Issues

- **Missing metrics**: Some sensors may not be available on all systems
- **Permissions**: Run as Administrator if certain metrics show "N/A"
- **Check logs**: Application logs errors to `dashboard.log` for debugging

## Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Fork and clone the repository**
2. **Install development dependencies**:
   ```bash
   rustup component add clippy rustfmt
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

### Project Structure

```
src/
├── main.rs      # Entry point, application initialization
├── model.rs     # Data structures for all metrics
├── hardware.rs  # Hardware polling and sensor interfacing
├── ui.rs        # GUI rendering and user interface
├── logger.rs    # Logging system
└── lib.rs       # Library exports
```

### Architecture

The application follows strict separation of concerns:

- **Data Model**: Defines metric data structures and shared state
- **Hardware Poller**: Runs in separate thread, polls sensors, updates model
- **UI Thread**: Renders interface, reads from model (one-way data flow)
- **Logger**: Handles error logging and debugging information

### Development Commands

```bash
# Run with development profile
cargo run

# Run tests
cargo test

# Run specific test
cargo test <test_name>

# Generate documentation
cargo doc --open

# Format code
cargo fmt

# Run linter
cargo clippy

# Clean build artifacts
cargo clean
```

### Performance Requirements

When contributing, ensure your changes maintain:
- **CPU Usage**: <2% on modern 4-core CPU
- **Memory Usage**: <100MB footprint
- **Efficiency**: No interference between monitoring and UI threads

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all clippy warnings (`cargo clippy`)
- Add tests for new functionality
- Update documentation for public APIs

### Submitting Changes

1. **Create a feature branch**: `git checkout -b feature/your-feature`
2. **Make your changes** following the coding standards
3. **Run tests**: `cargo test`
4. **Run linter**: `cargo clippy`
5. **Format code**: `cargo fmt`
6. **Commit with descriptive messages**
7. **Push and create a pull request**

### Issue Reporting

When reporting bugs, please include:
- Windows version and architecture
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Contents of `dashboard.log` if applicable

## License

[Add your license information here]

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) for the user interface
- Uses [sysinfo](https://github.com/GuillaumeGomez/sysinfo) for cross-platform system information
- Windows-specific sensors accessed via [windows-rs](https://github.com/microsoft/windows-rs)