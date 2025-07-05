# Project: Windows Performance Dashboard

## 1. Overview

A lightweight, real-time desktop application for Windows that monitors key hardware performance metrics. The application will display current, minimum, and maximum values for each metric from the start of the session and plot their history on a time-series graph. The application must prioritize computational and memory efficiency to minimize its own impact on system performance.

## 2. Core Functional Requirements

### 2.1. Data Polling & Display

The application shall poll for all metrics at a user-configurable interval, with a default of once per second (1Hz).

For each metric listed below, the application must display:

- **Current Value:** The most recently polled value.
- **Session Min:** The minimum value recorded since the application was launched.
- **Session Max:** The maximum value recorded since the application was launched.

### 2.2. Graphical Display

- For each metric, a real-time graph will plot the value's history over a rolling 60-second window.
- The Y-axis of each graph should dynamically scale to the session's Min/Max values for that metric.
- The X-axis will represent the 60-second time window.

### 2.3. Monitored Metrics

#### CPU Metrics

- Overall Utilization (%)
- Core Clock Speed (MHz)
- Package Temperature (°C)
- Hottest Core Temperature (°C)
- Thermal Throttling Status (Active/Inactive)

#### GPU Metrics (Primary GPU)

- Core Clock Speed (MHz)
- Package Temperature (°C)
- Hotspot Temperature (°C)
- Thermal Throttling Status (Active/Inactive)

#### Memory Metrics

- Overall RAM Utilization (%)
- RAM Utilization (MB)
- Memory Clock Speed (MHz)

#### System & Environmental Metrics

- Motherboard/Chipset Temperature (°C)
- Chassis/System Temperature (°C)
- All available system fan speeds (RPM), labeled clearly.
- All other available, uniquely-named temperature sensors (°C).

## 3. Non-Functional & Technical Requirements

### 3.1. Core Technology

- **Language:** Rust (latest stable version).
- **GUI Framework:** Use the `egui` crate for the user interface. It is well-suited for simple, immediate-mode GUIs and integrates well.
- **Hardware Interrogation:** Use the `liboem` or a similar comprehensive hardware monitoring library for Rust on Windows to fetch sensor data. This avoids direct OS-level calls.

### 3.2. Performance Constraints

- **CPU Usage:** The application's own CPU usage should not exceed 2% on a modern 4-core CPU.
- **Memory Usage:** The application's memory footprint should remain under 100MB.
- **Efficiency:** The application must be architected to ensure that the monitoring and UI rendering loops do not interfere with each other or introduce significant system load.

### 3.3. Error Handling

- If a sensor or metric is unavailable on the host system, display "N/A" instead of a value and disable its corresponding graph.
- The application should handle unexpected sensor read failures gracefully without crashing.

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
