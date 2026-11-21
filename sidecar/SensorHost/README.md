# SensorHost (optional)

Small elevated C# process hosting LibreHardwareMonitor. Exposes CPU/GPU temps and fan
speeds to WinIsland over a named pipe (`\\.\pipe\winisland-sensors`).

Why a sidecar: LibreHardwareMonitor is .NET and needs a ring-0 driver (admin).
Keeping it out-of-process means the main app stays small, unelevated and fast.
Off by default; enabled from Settings. Built in phase 8.
