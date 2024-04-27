Subject: https://cdn.intra.42.fr/pdf/pdf/60886/en.subject.pdf \
Build a renderer using either OpenGL, Vulkan, Metal or MinilibX (X11). \
Libraries are only allowed to manage windows and events (no parsing, no automated mesh creation, no vector/matrix/quaternion math, etc...). \
Colors / textures must be present on the faces of objects, and the objects must be able to rotate around it's center.

References used:
- Obj format: https://paulbourke.net/dataformats/obj/
- Mat format: https://en.wikipedia.org/wiki/Wavefront_.obj_file#Material_template_library
- Bmp format: https://en.wikipedia.org/wiki/BMP_file_format
- rust OpenGL usage (SDL2): https://rust-tutorials.github.io/learn-opengl/basics/001-drawing-a-triangle.html
- general OpenGL / GLSL usage: https://www.khronos.org/opengl/wiki / https://registry.khronos.org/OpenGL-Refpages/es3/

Rust version: `rustc 1.77.2`

Libraries used:
- `gl`: raw (unsafe) bindings to OpenGL (you can't really go more bare-bones than this)
- `winit`: window library, works on most platforms without having to ship a dll/lib
- `glutin`: used to create an OpenGL context for winit