Rust-wrapper for [tracy](https://bitbucket.org/wolfpld/tracy/).

From the official description:
> Tracy is a real time, nanosecond resolution frame profiler that can be used for remote or embedded telemetry of your application. It can profile CPU (C, C++11, Lua), GPU (OpenGL, Vulkan) and memory. It also can display locks held by threads and their interactions with each other.

Currently only the basic CPU profiling interface is exposed.
Disabling the "enable" feature currently doesn't work (i.e. will fail to build).

Requires a nightly toolchain.
