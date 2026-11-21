# Performance budget (targets, to be verified by profiling)

| Metric | Target | Measured |
|---|---|---|
| Idle CPU | ~0% | |
| Idle private memory | < 50 MB (expect 15-30 MB) | |
| Animation | 60 FPS, no dropped frames | |
| Startup to visible | < 300 ms | |
| Wakeups per second, idle | ~0 | |

Measure with Task Manager, Windows Performance Recorder / WPA, and PresentMon.
