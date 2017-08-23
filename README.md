# Aldaron's Device Interface - Graphical Processing Unit
Aldaron's Device Interface - Graphical Processing Unit, or adi_gpu for short, is
a library for interfacing with a gpu.  You can use it to render graphics onto a
surface.  Combined with awi, you can render to a window ( this is made easy with
adi_screen ).

## Backends
adi_gpu can use:
* Vulkan

adi_gpu will be able to use:
* OpenGL
* Metal
* Custom Drivers
* Imaginary GPU ( using CPU for GPU operations )

## Features
adi_gpu can:
* Render onto a window made with awi

adi_gpu will be able to:
* Render onto a surface
