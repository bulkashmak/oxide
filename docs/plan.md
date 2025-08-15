## Phase 1 – Project Setup & Core Infrastructure

| Cherno Episode        | What Cherno Does in C++            | Rust Equivalent Task                                                                                                                              |
| --------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 – Setup             | Create Hazel solution with Premake | Create a **Cargo workspace** with two crates: `engine` (lib) and `sandbox` (bin).                                                                 |
| 2 – Entry Point       | Add `main.cpp` in sandbox          | Add `main.rs` in sandbox that calls into engine `Engine::run()`.                                                                                  |
| 3 – Log System        | Use spdlog                         | Add [`log`](https://crates.io/crates/log) + [`env_logger`](https://crates.io/crates/env_logger) in `engine`, make `engine::log::init()` function. |
| 4 – Application Class | `Application` base class           | In Rust, define `pub struct Application` in `engine`, implement `run()` method.                                                                   |
| 5 – Entry Point Macro | `HZ_CREATE_APPLICATION` macro      | No macros needed — just call `engine::Application::new()` from `sandbox/main.rs`.                                                                 |

## Phase 2 - Windowing & Events

| Cherno Episode       | C++ Hazel                              | Rust Task                                                                                       |
| -------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------- |
| 6 – Window Class     | Use GLFW window abstraction            | Add `Window` struct wrapping [`winit`](https://crates.io/crates/winit) for window + event loop. |
| 7 – Event System     | Define `Event` base class & subclasses | Define `enum Event { KeyPressed, MouseMoved, WindowResize, ... }`.                              |
| 8 – Event Dispatcher | Type-safe dispatcher                   | Implement a simple event dispatch function using `match` instead of C++ RTTI.                   |
| 9 – Application Loop | Main loop calls `OnUpdate`             | In `Application::run()`, use winit’s `event_loop.run` to drive updates.                         |
| 10 – Layer Stack     | Push/pop layers                        | Define `Layer` trait in Rust and keep `Vec<Box<dyn Layer>>` as the stack.                       |

## Phase 3 – Rendering Setup

| Cherno Episode      | Hazel C++                       | Rust Task                                                                                                                 |
| ------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| 11 – OpenGL Context | Create OpenGL context with GLFW | In Rust, use `glow` with `winit` to create a GL context.                                                                  |
| 12 – Glad Loader    | Load OpenGL functions           | In Rust, `glow` handles this automatically.                                                                               |
| 13 – Clear Screen   | `glClearColor`                  | Call `gl.clear_color(...)` in Rust.                                                                                       |
| 14 – ImGui          | Dear ImGui integration          | Use [`imgui-rs`](https://crates.io/crates/imgui) + [`imgui-winit-support`](https://crates.io/crates/imgui-winit-support). |

## Phase 4 – ECS, Camera, Input

| Cherno Episode           | Hazel           | Rust Task                                                                                                       |
| ------------------------ | --------------- | --------------------------------------------------------------------------------------------------------------- |
| 15 – Input Class         | Poll GLFW keys  | Use `winit` events to track key/mouse state.                                                                    |
| 16 – Orthographic Camera | Manual matrices | Use [`glam`](https://crates.io/crates/glam) for math and matrix transforms.                                     |
| 17 – ECS Start           | Custom ECS      | Start with your own `struct Entity` + `Components` as Rust structs, or [`hecs`](https://crates.io/crates/hecs). |
| 18 – Render 2D Quad      | Simple renderer | Build a quad renderer with VAO/VBO in `glow`.                                                                   |

