# Simple Performance Dashboard - Test Suite Documentation

## Overview

This document describes the comprehensive unit and integration test suite for the Simple Performance Dashboard application. The test suite ensures the reliability, correctness, and performance of all application components.

## Test Structure

### 1. Unit Tests

#### Model Tests (`src/model.rs`)
- **MetricValue Testing**: 
  - Default initialization
  - Single and multiple value updates
  - Min/max tracking across different data types
  - Boolean metric handling (thermal throttling)
  - History maintenance and chronological ordering
  - Edge cases (zero, negative, extreme values)
  - Plot data generation and timestamp conversion

- **Data Structure Testing**:
  - CPU, GPU, Memory, Storage, and Motherboard metrics initialization
  - Default values verification
  - Data availability detection methods
  - UI state management

- **AppState Testing**:
  - Default and custom initialization
  - Shared state creation and concurrent access
  - Session timing verification
  - Polling interval configuration

- **ToF64 Trait Testing**:
  - Type conversion accuracy for f32, u32, u64, bool
  - Plot data generation for different metric types

#### Hardware Tests (`src/hardware.rs`)
- **HardwarePoller Testing**:
  - Poller creation and configuration
  - Individual metric update methods (CPU, GPU, Memory, Storage, Motherboard)
  - Comprehensive hardware polling cycles
  - Temperature sensor detection and validation
  - Metric bounds validation (percentages, frequencies, temperatures)
  - Concurrent access safety
  - Error handling robustness

- **Error Handling Testing**:
  - HardwareError enum variants and formatting
  - HardwareResult type functionality
  - Error trait implementation
  - Graceful degradation on sensor failures

#### Logger Tests (`src/logger.rs`)
- **AppLogger Testing**:
  - Logger creation and file initialization
  - Different log levels (info, warning, error)
  - Sensor-specific logging functions
  - Session header formatting
  - File rewriting behavior between sessions
  - Timestamp formatting validation

- **Global Logger Testing**:
  - Global logger initialization
  - Thread-safe logging operations
  - Behavior without initialization (graceful degradation)
  - Concurrent logging from multiple threads

- **File System Testing**:
  - Log file creation and cleanup
  - Path validation and error handling
  - Content verification and ordering

### 2. Integration Tests (`tests/integration_tests.rs`)

#### End-to-End Data Flow
- Complete data collection and state update cycle
- Hardware polling → State update → UI data availability
- Multi-cycle polling with history accumulation
- Min/max tracking verification across polling cycles

#### Concurrent Operations
- Simultaneous hardware polling and UI data access
- Thread safety verification under concurrent load
- Data consistency during high-frequency updates
- Reader-writer lock behavior validation

#### UI Integration
- Data availability detection for all metric categories
- Plot data generation and interpolation
- Crosshair functionality with data value interpolation
- Session timing consistency

#### Application Lifecycle
- Complete startup → polling → runtime → shutdown simulation
- State persistence across operations
- Memory usage and performance under load
- Data consistency verification

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test Categories
```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests

# Specific module tests
cargo test model::tests
cargo test hardware::tests
cargo test logger::tests
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests in Release Mode
```bash
cargo test --release
```

## Test Coverage

### Model Module Coverage
- ✅ MetricValue lifecycle (creation, updates, history)
- ✅ Data type handling (f32, u32, u64, bool)
- ✅ Min/max tracking accuracy
- ✅ Plot data generation
- ✅ All metric structures (CPU, GPU, Memory, Storage, Motherboard)
- ✅ AppState functionality and data availability detection
- ✅ Concurrent access patterns
- ✅ Edge cases and error conditions

### Hardware Module Coverage
- ✅ HardwarePoller initialization and configuration
- ✅ Individual metric collection methods
- ✅ System information refresh operations
- ✅ Temperature sensor detection
- ✅ Error handling and graceful degradation
- ✅ Bounds validation for all metric types
- ✅ Concurrent polling operations
- ✅ Thread safety verification

### Logger Module Coverage
- ✅ AppLogger creation and file operations
- ✅ All logging levels and functions
- ✅ Global logger initialization and usage
- ✅ Thread-safe logging operations
- ✅ File system interactions
- ✅ Session management and file rewriting
- ✅ Error condition handling

### Integration Coverage
- ✅ End-to-end data flow verification
- ✅ Concurrent operations safety
- ✅ UI data access patterns
- ✅ Application lifecycle simulation
- ✅ Performance under load
- ✅ Data consistency verification

## Test Environment Requirements

### System Requirements
- Rust 1.70+ with cargo test support
- Windows, macOS, or Linux (cross-platform testing)
- Read/write access to temporary directories for log file testing
- System information access for hardware polling tests

### Test Data
- Tests use controlled, deterministic data where possible
- Real system metrics are used for integration tests
- Temporary files are created and cleaned up automatically
- No persistent test artifacts remain after test completion

## Continuous Integration

### Test Automation
The test suite is designed for automated CI/CD environments:
- No user interaction required
- Deterministic test outcomes
- Comprehensive error reporting
- Clean setup and teardown

### Performance Benchmarks
Key performance metrics verified by tests:
- Memory usage stays within bounds during testing
- Polling operations complete within expected timeframes
- Concurrent access doesn't cause deadlocks or race conditions
- UI responsiveness maintained under load

## Test Maintenance

### Adding New Tests
When adding new functionality:
1. Add unit tests for individual components
2. Add integration tests for end-to-end workflows
3. Update this documentation
4. Verify test coverage remains comprehensive

### Test Data Updates
- Mock data should reflect realistic hardware values
- Test bounds should cover expected operational ranges
- Edge cases should include both normal and extreme conditions

## Troubleshooting

### Common Test Issues
1. **File Permission Errors**: Ensure write access to temp directories
2. **Timing Issues**: Tests include appropriate delays for system operations
3. **Platform Differences**: Tests account for cross-platform variations
4. **Resource Cleanup**: All tests clean up temporary resources

### Test Debugging
- Use `cargo test -- --nocapture` for detailed output
- Individual test execution: `cargo test test_name`
- Debug builds include additional assertion information
- Log files can be inspected during test failures

## Quality Assurance

### Test Quality Metrics
- **Coverage**: All public APIs have corresponding tests
- **Reliability**: Tests pass consistently across platforms
- **Performance**: Test execution completes within reasonable time
- **Maintainability**: Tests are clear, documented, and updatable

### Verification Standards
- All tests must pass before code merge
- New features require corresponding test coverage
- Performance regressions are caught by integration tests
- Memory leaks and resource issues are detected during testing

This comprehensive test suite ensures the Simple Performance Dashboard maintains high quality, reliability, and performance across all supported platforms and use cases.