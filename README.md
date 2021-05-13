
Rust port of https://github.com/JoeyDeVries/LearnOpenGL

You should be able to follow the tutorials at https://learnopengl.com/ with this - the code structure has been kept similar to the original C++ but also  tried to follow the rust style guidelines as much as possible. 

> However, it's not necessarily the most idiomatic Rust code.All OpenGL calls are only abstracted by glow and wrapped in `unsafe` blocks.
If you want a rust implementation that will closer follow the C++ tutorial please see the [learn-opgenl-rs](https://github.com/bwasty/learn-opengl-rs) github repository.

Run individual tutorials like this:
`cargo run 1_3_2` (for `/src/_1_getting_started/_3_2_shaders_interpolation.rs`).

For reduced compilation times, you may only compile the code for a certain chapter by adding `--no-default-features --features chapter-1` for example.
<p align="center">
<a href="src/_1_getting_started/_2_1_hello_triangle.rs"><img width="250" alt="1_3_2" title="1_3_2 Hello Triangle" src="https://user-images.githubusercontent.com/1647415/27755053-d5cd0f5a-5ded-11e7-99b4-abd4e3bb8638.png"></a>
<a href="src/_2_lighting/_6_multiple_lights.rs"><img width="250" alt="2_6" title="2_6 Multiple Lights" src="https://user-images.githubusercontent.com/1647415/27755102-fd217078-5ded-11e7-96f6-efdeb9ffdcac.png"></a>
<a href="src/_3_model_loading/_1_model_loading.rs"><img width="250" alt="3_1" title="3_1 Model Loading"src="https://user-images.githubusercontent.com/1647415/27755660-52df4104-5df1-11e7-800c-45a514bf3130.png"></a>
</p>
<p align="center">
<a href="src/_4_advanced_opengl/_6_2_cubemaps_environment_mapping.rs"><img width="250" alt="4_6_2" title="4_6_2 Framebuffers"src="https://user-images.githubusercontent.com/1647415/27843160-306a96aa-6111-11e7-8b89-15820f39cff0.png"></a>
<a href="src/_4_advanced_opengl/_9_1_geometry_shader_houses.rs"><img width="250" alt="4_9_1" title="4_9_1 Geometry Shader"src="https://user-images.githubusercontent.com/1647415/28338597-c1fa9ed2-6c09-11e7-9e25-3e70e6fbacd9.png"></a>
<a href="src/_4_advanced_opengl/_10_3_asteroids_instanced.rs"><img width="250" alt="4_10_3" title="4_10_3 Instancing"src="https://user-images.githubusercontent.com/1647415/28338123-3748ea6a-6c08-11e7-9c50-93f333a15083.png"></a>
</p>

## Chapters
### [1. Getting started](src/_1_getting_started)
**Notes**
- You can mostly ignore the setup instructions at [Getting-started/Creating-a-window](https://learnopengl.com/#!Getting-started/Creating-a-window). Just create a new project with `cargo` and copy the dependency section from [Cargo.toml](Cargo.toml). The `glow` library is used to access the OpenGL API [(see details)](https://github.com/grovesNL/glow), in addition [glutin](https://github.com/tomaka/glutin) library is used to host the OpenGL application.
- If you experience black screens or weird rendering artifacts, check out the [`glCheckError!`](https://github.com/bwasty/learn-opengl-rs/blob/89aed9919a2347e49965820830a6aecfdda18cf3/src/_7_in_practice/_1_debugging.rs#L28-L53) macro from chapter 7.
- exercises have been mostly omitted. You can look up the solutions in the original C++ source (although it is still on my todo list).

### [2. Lighting](src/_2_lighting)
### [3. Model loading](src/_3_model_loading)
**Notes**
- For simplicity [`tobj`](https://github.com/Twinklebear/tobj) is used instead of `assimp` (simpler interface, pure Rust and later tutorials only load OBJ files anyway). For alternatives see [here](http://arewegameyet.com/categories/3dformatloader.html) and [here](https://crates.io/search?q=assimp).
- The `image` crate is quite slow in debug mode - loading the nanosuit textures takes so much time that it can be faster to use release mode (including compile time).
### [4. Advanced OpenGL](src/_4_advanced_opengl)
**Status:** in progress
### [5. Advanced Lighting](src/_5_advanced_lighting)
**Status:** todo.
### [6. PBR](src/_6_pbr)
**Status:** todo.
### [7. In Practice](src/_7_in_practice)
**Status:** `Debugging` in progress (the other two are not in the repo)

----
### A note about the code organization
Now samples are intgrated in one big main which can lead to long compile times. As a workaround there is a config feature flags for each chapter.

