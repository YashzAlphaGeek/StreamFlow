# **StreamFlow: Adaptive Data Stream Processing**

**StreamFlow** is a real-time data streaming system designed to efficiently process and display system resource usage metrics. It leverages dynamic polling intervals, ensuring the system consumes resources efficiently while displaying up-to-date information for CPU, memory, and disk usage.

---

## **Concept**

StreamFlow intelligently adjusts the polling frequency for system resource data (CPU, memory, disk) based on configurable intervals. It aims to provide accurate and real-time metrics while maintaining efficiency in terms of system resources, making it ideal for continuous monitoring of system performance.

---

## **StreamFlow Project Architecture**

Below is the architecture of the StreamFlow project:

```mermaid
graph LR
    title StreamFlow Project Architecture
    A[StreamFlow Project] --> B[src]
    B --> C[main.rs]
    B --> D[terminal_ui.rs]
    B --> E[data_provider.rs]
    B --> F[data_receiver.rs]
    B --> G[interval_support.rs]
    B --> H[stream_service.rs]

    subgraph TerminalUI
        D --> I[Terminal UI Logic]
    end

    subgraph DataProvider
        E --> J[Data Handling]
    end

    subgraph DataReceiver
        F --> K[Data Input/Receiver Logic]
    end

    subgraph IntervalSupport
        G --> L[Interval Optimization]
    end

    subgraph StreamService
        H --> M[Main Stream Logic]
    end

    style A fill:#f2f2f2,stroke:#333,stroke-width:2px
    style B fill:#e0e0e0,stroke:#333,stroke-width:2px
    style C fill:#c2d9f7,stroke:#333,stroke-width:2px
    style D fill:#c3e6cb,stroke:#333,stroke-width:2px
    style E fill:#e2e3e5,stroke:#333,stroke-width:2px
    style F fill:#e2e3e5,stroke:#333,stroke-width:2px
    style G fill:#c3e6cb,stroke:#333,stroke-width:2px
    style H fill:#e2e3e5,stroke:#333,stroke-width:2px
    style I fill:#e3f2fd,stroke:#333,stroke-width:2px
    style J fill:#f5c6cb,stroke:#333,stroke-width:2px
    style K fill:#f5c6cb,stroke:#333,stroke-width:2px
    style L fill:#f5c6cb,stroke:#333,stroke-width:2px
    style M fill:#f5c6cb,stroke:#333,stroke-width:2px
    
---

## **Key Features:**

- **Dynamic Polling Frequency**: Configurable polling intervals, with support for adjustments to optimize performance.
- **Efficient Resource Usage**: Continuously monitors system resource usage without excessive consumption of CPU or memory.
- **Real-Time Updates**: Displays up-to-date system metrics (CPU, memory, disk usage) at regular intervals.
- **Simple Terminal UI**: A straightforward terminal-based user interface for displaying resource usage in text-based graphs.

---

## **Technologies Used**

- **Rust**: The project is built with Rust for high-performance, concurrency, and low-latency data processing.
- **Real-Time Data Processing**: Efficiently fetches and updates system data for CPU, memory, and disk usage.
- **Cross-Platform**: Works across different platforms using the `sysinfo` crate to gather system metrics.
- **RatUI**: Uses `ratatui` for a simple terminal user interface to display the data in a structured format.

---

## **Getting Started**

### Prerequisites:

- **Rust**: Install Rust from [Rust Installation](https://www.rust-lang.org/tools/install).

### Steps to run:

1. Clone the repository:
   ```bash
   git clone https://github.com/YashzAlphaGeek/StreamFlow.git
   cd StreamFlow
2. Cargo Build:
   ```bash
   cargo build
3. Cargo Run:
   ```bash
   cargo run