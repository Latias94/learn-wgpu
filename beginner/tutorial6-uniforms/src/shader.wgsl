// 顶点着色器
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

// 因为我们已经创建了一个新的绑定组，所以需要指定在着色器中使用哪一个。这个数字由我们的 render_pipeline_layout 决定。
// texture_bind_group_layout 被列在第一位，因此它是 group(0)，而 camera_bind_group 是第二位，因此它是 group(1)。
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    // 当涉及到矩阵时，乘法的顺序很重要。向量在最右边，矩阵按重要性顺序在左边（裁剪空间坐标 = 投影矩阵 x 模型视图矩阵 x 位置向量）。
    out.clip_position = camera.view_proj *  vec4<f32>(model.position, 1.0);
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
