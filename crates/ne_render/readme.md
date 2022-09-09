[features]
print_fps = []
first_frame_time = []
start_time = []

example:
ne_render = {path = "../../crates/ne_render", features = ["print_fps"] }



Renderer is not tested for browser. In the future make another version ne_renderer3D_wasm

"Don't use block_on inside of an async function if you plan to support WASM. Futures have to be run using the browser's executor. If you try to bring your own your code will crash when you encounter a future that doesn't execute immediately."
- https://sotrh.github.io/learn-wgpu/beginner/tutorial2-surface/#state-new

