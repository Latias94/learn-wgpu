# Learn Wgpu

跟着 [Learn Wgpu](https://sotrh.github.io/learn-wgpu/) 对应的 [中文版](https://jinleili.github.io/learn-wgpu-zh/) 敲。

[Wgpu](https://github.com/gfx-rs/wgpu) 是 [WebGPU API](https://gpuweb.github.io/gpuweb/) 规范的一个 Rust 实现。

```shell
# 在桌面环境本地运行
cargo run --example tutorial2-surface
# 在浏览器中运行：使用 WebGPU（需要使用 FireFox Nightly 或 Chrome Canary 并开启 WebGPU 试验功能）
cargo run-wasm --example tutorial1-window
# 在浏览器中运行：使用 WebGL 2.0
cargo run-wasm --example tutorial2-surface --features webgl
```
