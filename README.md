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


## Sources

- Amdahl, Gene M. (1967). *Validity of the Single Processor Approach to Achieving Large Scale Computing Capabilities*. Proceedings of the AFIPS Spring Joint Computer Conference, 483–485. https://doi.org/10.1145/1465482.1465560

- Algazi, V. Ralph; Duda, Richard O.; Thompson, Dennis M.; Avendano, Carlos (2001). *The CIPIC HRTF Database*. Proceedings of the IEEE Workshop on Applications of Signal Processing to Audio and Acoustics (WASPAA), 99–102. https://doi.org/10.1109/ASPAA.2001.969552

- Audio Engineering Society (2022). *AES69-2022: AES Standard for File Exchange — Spatial Acoustic Data File Format (SOFA)*. Audio Engineering Society, New York, NY.

- Kuttruff, Heinrich (2016). *Room Acoustics* (6th ed.). CRC Press, Boca Raton, FL.

- Möller, Tomas; Trumbore, Ben (1997). *Fast, Minimum Storage Ray-Triangle Intersection*. Journal of Graphics Tools, 2(1), 21–28. https://doi.org/10.1080/10867651.1997.10487468

- Oppenheim, Alan V.; Schafer, Ronald W. (2010). *Discrete-Time Signal Processing* (3rd ed.). Pearson, Upper Saddle River, NJ.

- Perez-Lopez, Andres. *pysofaconventions: Python API for reading and writing SOFA files*. https://github.com/andresperezlopez/pysofaconventions

- Pharr, Matt; Jakob, Wenzel; Humphreys, Greg (2023). *Physically Based Rendering: From Theory to Implementation* (4th ed.). MIT Press, Cambridge, MA.

- SciPy Developers. *scipy.signal.oaconvolve documentation*. https://docs.scipy.org/doc/scipy/reference/generated/scipy.signal.oaconvolve.html

- SOFA Acoustics. *CIPIC HRTF Database in SOFA format, database_sofa_0.6*. https://sofacoustics.org/data/database_sofa_0.6/cipic/

- Virtanen, Pauli; Gommers, Ralf; Oliphant, Travis E.; et al. (2020). *SciPy 1.0: Fundamental Algorithms for Scientific Computing in Python*. Nature Methods, 17(3), 261–272. https://doi.org/10.1038/s41592-019-0686-2

- Vorländer, Michael (2008). *Auralization: Fundamentals of Acoustics, Modelling, Simulation, Algorithms and Acoustic Virtual Reality*. Springer, Berlin, Heidelberg.
