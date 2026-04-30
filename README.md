# Distributed Acoustic Ray Tracer

## Overview
This project implements a distributed acoustic ray tracing system to simulate realistic sound propagation in 3D environments.

The system uses a cluster-based architecture to parallelize the computation of sound rays and generate impulse responses (IR), which are later used for audio convolution.

---

## Architecture

### 1. Physics Engine (Rust)
- Simulates sound rays in a 2D/3D environment
- Handles reflections, energy loss, and intersections
- Outputs impulse responses (IR)

### 2. Distributed System (Python + Ray/Dask)
- Splits ray simulation into parallel tasks
- Distributes workload across a compute cluster
- Aggregates partial IR results

### 3. DSP Engine (Python)
- Applies convolution of IR with audio signals
- Produces final reverberated audio output

---

## Goals
- Efficient parallelization of ray tracing workloads
- Scalability across multi-core and cluster environments
- Performance benchmarking and analysis

---

## Tech Stack
- Rust (high-performance simulation)
- Python (orchestration, DSP)
- Ray / Dask (distributed computing)

---

## Project Structure (planned)
