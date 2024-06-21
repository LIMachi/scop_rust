Subject: https://cdn.intra.42.fr/pdf/pdf/131529/fr.subject.pdf \
Build a renderer using either OpenGL, Vulkan, Metal or MinilibX (X11). \
Libraries are only allowed to manage windows and events (no parsing, no automated mesh creation, no vector/matrix/quaternion math, etc...). \
Colors / textures must be present on the faces of objects, and the objects must be able to rotate around it's center.

References used:
- Obj format: https://paulbourke.net/dataformats/obj/
- Mat format: https://en.wikipedia.org/wiki/Wavefront_.obj_file#Material_template_library
- Bmp format: https://en.wikipedia.org/wiki/BMP_file_format
- rust OpenGL usage (SDL2): https://rust-tutorials.github.io/learn-opengl/basics/001-drawing-a-triangle.html
- opengl techniques: https://learnopengl.com/Introduction
- general OpenGL / GLSL usage: https://www.khronos.org/opengl/wiki / https://registry.khronos.org/OpenGL-Refpages/es3/

Rust version: `rustc 1.77.2`

Libraries used:
- `gl`: raw (unsafe) bindings to OpenGL (you can't really go more bare-bones than this)
- `winit`: window library, works on most platforms without having to ship a dll/lib
- `glutin`: used to create an OpenGL context for winit

Completion:
- [x] obj parsing
- [x] mtl parsing
- [x] bmp parsing
- [x] windowing
- [x] perspective
- [ ] centered rotation
- [x] colors / shades of grey to distinguish faces
- [ ] object can move in any directions -> need to add inputs to control object instead of camera
- [ ] application of texture -> only works with mtl declared textures for now
- [ ] soft transition between colored and textured faces
- [x] reimplement matrices and vectors (even quaternions)

Bonuses:
- [ ] handle teapot (missing materials, missing groups)
- [ ] non stretched textures on objects without uvs/materials
- [ ] free camera movement
- [x] multiples objects
- [x] instancing
- [ ] lights
- [ ] shadows

Testing (currently disable for rework):
- Key binds:
- - W / A / S / D -> horizontal camera movement
- - Space / Left Shift -> vertical camera movement
- - Q / E -> roll
- - Up / Down / Mouse Up / Mouse Down -> pitch
- - Left / Right / Mouse Left / Mouse Right -> yaw
- - Ctrl -> toggle fast camera movement / rotation
- - R -> toggle object rotation
- - F -> toggle between colored and textured faces
- - M -> toggle between full faces, lines and dots
- - Todo:
- - - left click: take control of aimed object
- - - right click: stop controlling object
- Executable parameters:
- - any amount of paths to objects (extension can be omitted, path can be partial and will be tested against this executable position and 'resources' folder)
- - example: `cargo run --release -- 42 dragon` will spawn both the 42 from ./resources/objs/42.obj and the dragon.obj next to it
- - example: `cargo run --release -- ../my_object` will find any `my_object.obj` in either the folder above the command, next to the command or in resources (but not in the subfolders of resources)