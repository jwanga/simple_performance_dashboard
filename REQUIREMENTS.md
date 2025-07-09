# Project: Simple Performance Dashboard

## 1. Overview

A lightweight, real-time desktop application that monitors key hardware performance metrics. The application will display current, minimum, and maximum values for each metric from the start of the session and plot their history on a time-series graph. The application must prioritize computational and memory efficiency to minimize its own impact on system performance.

## 2. Core Functional Requirements

### 2.1. Data Polling & Display

The application shall poll for all metrics at a user-configurable interval, with a default of once per second (1Hz).

For each metric listed below, the application must display:

- **Current Value:** The most recently polled value.
- **Session Min:** The minimum value recorded since the application was launched.
- **Session Max:** The maximum value recorded since the application was launched.
- **Time-Series Graph:** A real-time graph plotting the metric's history over the session duration.

**IMPORTANT**: Every single metric must include all four components above (current/min/max values plus graph). This applies to all metric types including binary status indicators (e.g., thermal throttling status), numeric values, temperatures, speeds, and any other monitored parameters. No exceptions.

### 2.2. Graphical Display

#### Graph Axis Bounds (Value Ranges)
  - Y-axis bounds: dynamically adjust to session minimum and maximum values
  - X-axis bounds: 0 to elapsed time since session started (seconds)

#### Graph Scaling (Fit Behavior)
  - Graph scales to fit fixed widget size without scrolling
  - All data remains visible within widget dimensions
  - Widget size stays constant, graph content stretches/shrinks to fit

#### Graph Interaction
  - **Crosshair Display**: When hovering over a graph, display a crosshair with data values
  - **X-axis value**: Show elapsed time in seconds since session start at cursor position
  - **Y-axis value**: Show the actual metric value (with appropriate units) at cursor position
  - **Data interpolation**: If cursor is between data points, interpolate to show estimated value at that time
  - **Unit formatting**: Display values with proper units (%, MHz, °C, RPM, MB) matching the metric type

### 2.3. Monitored Metrics

#### CPU Metrics

- CPU Utilization (%)
- CPU Core Clock Speed (MHz)
- CPU Core Voltage (V)
- CPU Power Consumption (W)
- CPU Temperature (°C)
- CPU Hottest Core Temperature (°C)
- CPU Thermal Throttling Status (1=Active/0=Inactive)

#### GPU Metrics (Primary GPU)

- GPU Utilization (%)
- Core Clock Speed (MHz)
- GPU Memory Utilization (MB)
- GPU Core Voltage (V)
- GPU power consumption (W)
- GPU Temperature (°C)
- GPU Hotspot Temperature (°C)
- GPU Thermal Throttling Status (1=Active/0=Inactive)

#### Memory Metrics

- Memory Utilization (MB)
- Memory Clock Speed (MHz)
- Memory Temperature (°C)

#### Storage Metrics

- Drive read speed (MB/s)
- Drive write speed (MB/s)
- Drive temperature (°C)

#### Motherboard Metrics
- Chipset Temperature (°C)
- Chassis Temperature (°C)
- AIO Pump Speed (RPM)
- Chassis Fan Speed (RPM)
- Chipset Fan Speed (RPM)

### 2.4: User Interface Organization

  ##### Metric Grouping

  - All metrics must be organized into logical, collapsible sections:
    - CPU Metrics: All CPU-related metrics.
    - GPU Metrics: All GPU-related metrics.
    - Memory Metrics: All memory-related metrics.
    - Storage Metrics: All storage-related metrics.
    - Motherboard Metrics: All motherboard-related metrics.

  #### Collapsible Section Behavior

  - Each metric section must be implemented as a collapsible/expandable panel
  - Section headers display the category name with expand/collapse indicator (▼/▶)
  
  ##### Default Expansion States (Data-Aware)
  - **Sections with available data**: Must be expanded by default, allowing users to immediately see all available metrics
  - **Sections without available data**: Must be collapsed by default, with grayed-out titles and "(No Data)" suffix to clearly indicate unavailability
  - **Dynamic behavior**: As the application collects data from sensors, sections automatically transition from collapsed/grayed to expanded/normal appearance
  
  ##### User Interaction
  - Users can click section headers to manually toggle visibility of any section regardless of data availability
  - Manual user choices override the default data-aware behavior
  
  ##### Visual Indicators
  - **Available data**: White text, normal section title, expanded by default
  - **No data**: Gray text, section title with "(No Data)" suffix, collapsed by default
  - Clear expand/collapse indicators (▼/▶) provided by the UI framework

  #### Section Layout

  - Within each expanded section, metrics should be displayed in a 2-column grid layout
  - Each metric occupies one column position with:
    - Metric name as header
    - Current/Min/Max value display
    - Time-series graph below values
  - **ALL metrics must follow this layout consistently**, including binary status indicators (thermal throttling, etc.) which must display their current/min/max states and graph their history over time
  - Each metric should have a consistent visual style for headers, values, and graphs regardless of data type

  #### Visual Hierarchy

  - Section headers should be visually distinct from metric headers
  - Consistent spacing and grouping within each section
  - Clear separation between different metric sections

## 3. Non-Functional & Technical Requirements

### 3.1. Core Technology

- **Language:** Rust (latest stable version).
- **GUI Framework:** Use the `egui` crate for the user interface. It is well-suited for simple, immediate-mode GUIs and integrates well.
- **Hardware Interrogation:** Use the `liboem` or a similar comprehensive hardware monitoring library for Rust on Windows to fetch sensor data. This avoids direct OS-level calls.

### 3.2. Vendor Compatibility
- The application must support Intel, AMD and Apple Silicon CPUs, Make sure you implement all appropriate API's and detect which ones to use at run time.
- The application must support NVIDIA, AMD, Intel and Apple Silicon GPUs, Make sure you implement all appropriate API's and detect which ones to use at run time.
- Do not leave any metric unsupported. Research the appropriate libraries and APIs to ensure all metrics can be read on all supported hardware.

### 3.3. Performance Constraints

- **CPU Usage:** The application's own CPU usage should not exceed 2% on a modern 4-core CPU.
- **Memory Usage:** The application's memory footprint should remain under 100MB.
- **Efficiency:** The application must be architected to ensure that the monitoring and UI rendering loops do not interfere with each other or introduce significant system load.

### 3.. Error Handling

- If a sensor or metric is unavailable on the host system, display "N/A" instead of a value and disable its corresponding graph.
If a sensor or metric is unavailable on the host system, display the graph without data. The X-axis of the graph without data should still scale with elapsed time since the session started.
- The application should handle unexpected sensor read failures gracefully without crashing.
- save all errors to a log file in the application directory for debugging purposes. Rewrite the log file on each session start to avoid accumulating old errors.

## 4. Logical Architecture

### 4.1. Separation of Concerns

The application logic must be strictly separated from the presentation layer.

- **Data Model (`/src/model.rs`):** A module responsible for defining the data structures for all metrics.
- **Hardware Poller (`/src/hardware.rs`):** A separate module running in its own thread that is solely responsible for interfacing with the hardware monitoring library, polling for new data on a timer, and updating the data model.
- **UI / Presentation (`/src/ui.rs` or `/src/main.rs`):** The main thread, responsible for rendering the `egui` interface. It should read from the data model but not modify it directly. This follows a one-way data flow pattern.

### 4.2. Project Structure

```
/src
|-- main.rs      # Entry point, sets up egui window and threads
|-- model.rs     # Defines data structs for all metrics
|-- hardware.rs  # Logic for polling hardware sensors
|-- ui.rs        # All egui rendering logic
```
