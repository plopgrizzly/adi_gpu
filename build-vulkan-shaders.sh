# Aldaron's Device Interface / GPU                                             #
# Copyright (c) 2017 Plop Grizzly, Jeron Lau <jeron.lau@plopgrizzly.com>       #
# Licensed under the MIT LICENSE                                               #
#                             _        _                                       #
#                            /@\——————/@\                                      #
# .———  .                   |   o    o   |     .————      .            .       #
# |   | |  .———   .———   .——.     []     .——.  |      .——    ———: ———: | .   . #
# |   | |  |   |  |   |  \   \   <##>   /   /  |      |   |    /    /  | |   | #
# |———  |  |   |  |   |   |   ¯        ¯   |   |   -- |   |   /    /   |  \ /  #
# |     |  |   |  |———    |                |   |    | |   |  /    /    |   |   #
# |     |   ———   |       |                |    ————  |   | :——— :———  |   |   #
#                 |        \              /                              __/   #
#                           ¯————————————¯                                     #
# gen-spirv.sh                                                                 #

SPIRV_OPT="spirv-opt --strip-debug --freeze-spec-const --eliminate-dead-const --fold-spec-const-op-composite --unify-const"
SRC=src/native_renderer/vulkan/glsl

OUT_UNOPTIMIZED=target/spv/unoptimized
OUT_OPTIMIZED=target/spv/optimized
OUT_RELEASE=target/spv/release

mkdir -p $OUT_UNOPTIMIZED/
mkdir -p $OUT_OPTIMIZED/
mkdir -p $OUT_RELEASE/

glslangValidator $SRC/solid-frag.glsl -V -o $OUT_UNOPTIMIZED/solid-frag.spv -S frag
glslangValidator $SRC/solid-vert.glsl -V -o $OUT_UNOPTIMIZED/solid-vert.spv -S vert
glslangValidator $SRC/solid-nafrag.glsl -V -o $OUT_UNOPTIMIZED/solid-nafrag.spv -S frag
glslangValidator $SRC/solid-bfrag.glsl -V -o $OUT_UNOPTIMIZED/solid-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-frag.spv -o $OUT_OPTIMIZED/solid-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-vert.spv -o $OUT_OPTIMIZED/solid-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-nafrag.spv -o $OUT_OPTIMIZED/solid-nafrag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/solid-bfrag.spv -o $OUT_OPTIMIZED/solid-bfrag.spv

glslangValidator $SRC/gradient-frag.glsl -V -o $OUT_UNOPTIMIZED/gradient-frag.spv -S frag
glslangValidator $SRC/gradient-vert.glsl -V -o $OUT_UNOPTIMIZED/gradient-vert.spv -S vert
glslangValidator $SRC/gradient-nafrag.glsl -V -o $OUT_UNOPTIMIZED/gradient-nafrag.spv -S frag
glslangValidator $SRC/gradient-bfrag.glsl -V -o $OUT_UNOPTIMIZED/gradient-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-frag.spv -o $OUT_OPTIMIZED/gradient-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-vert.spv -o $OUT_OPTIMIZED/gradient-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-nafrag.spv -o $OUT_OPTIMIZED/gradient-nafrag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/gradient-bfrag.spv -o $OUT_OPTIMIZED/gradient-bfrag.spv

glslangValidator $SRC/texture-frag.glsl -V -o $OUT_UNOPTIMIZED/texture-frag.spv -S frag
glslangValidator $SRC/texture-vert.glsl -V -o $OUT_UNOPTIMIZED/texture-vert.spv -S vert
glslangValidator $SRC/texture-nafrag.glsl -V -o $OUT_UNOPTIMIZED/texture-nafrag.spv -S frag
glslangValidator $SRC/texture-bfrag.glsl -V -o $OUT_UNOPTIMIZED/texture-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-frag.spv -o $OUT_OPTIMIZED/texture-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-vert.spv -o $OUT_OPTIMIZED/texture-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-nafrag.spv -o $OUT_OPTIMIZED/texture-nafrag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/texture-bfrag.spv -o $OUT_OPTIMIZED/texture-bfrag.spv

glslangValidator $SRC/faded-frag.glsl -V -o $OUT_UNOPTIMIZED/faded-frag.spv -S frag
glslangValidator $SRC/faded-vert.glsl -V -o $OUT_UNOPTIMIZED/faded-vert.spv -S vert
glslangValidator $SRC/faded-frag.glsl -V -o $OUT_UNOPTIMIZED/faded-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/faded-frag.spv -o $OUT_OPTIMIZED/faded-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/faded-vert.spv -o $OUT_OPTIMIZED/faded-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/faded-frag.spv -o $OUT_OPTIMIZED/faded-bfrag.spv

glslangValidator $SRC/tinted-frag.glsl -V -o $OUT_UNOPTIMIZED/tinted-frag.spv -S frag
glslangValidator $SRC/tinted-vert.glsl -V -o $OUT_UNOPTIMIZED/tinted-vert.spv -S vert
glslangValidator $SRC/tinted-nafrag.glsl -V -o $OUT_UNOPTIMIZED/tinted-nafrag.spv -S frag
glslangValidator $SRC/tinted-nafrag.glsl -V -o $OUT_UNOPTIMIZED/tinted-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/tinted-frag.spv -o $OUT_OPTIMIZED/tinted-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/tinted-vert.spv -o $OUT_OPTIMIZED/tinted-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/tinted-nafrag.spv -o $OUT_OPTIMIZED/tinted-nafrag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/tinted-nafrag.spv -o $OUT_OPTIMIZED/tinted-bfrag.spv

glslangValidator $SRC/complex-frag.glsl -V -o $OUT_UNOPTIMIZED/complex-frag.spv -S frag
glslangValidator $SRC/complex-vert.glsl -V -o $OUT_UNOPTIMIZED/complex-vert.spv -S vert
glslangValidator $SRC/complex-nafrag.glsl -V -o $OUT_UNOPTIMIZED/complex-nafrag.spv -S frag
glslangValidator $SRC/complex-bfrag.glsl -V -o $OUT_UNOPTIMIZED/complex-bfrag.spv -S frag
$SPIRV_OPT $OUT_UNOPTIMIZED/complex-frag.spv -o $OUT_OPTIMIZED/complex-frag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/complex-vert.spv -o $OUT_OPTIMIZED/complex-vert.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/complex-nafrag.spv -o $OUT_OPTIMIZED/complex-nafrag.spv
$SPIRV_OPT $OUT_UNOPTIMIZED/complex-bfrag.spv -o $OUT_OPTIMIZED/complex-bfrag.spv

spirv-remap --map all --dce all --strip-all --input $OUT_OPTIMIZED/*.spv --output $OUT_RELEASE/

cp $OUT_RELEASE/* src/native_renderer/vulkan/res/
# cp $OUT_UNOPTIMIZED/* src/native_renderer/vulkan/res/
