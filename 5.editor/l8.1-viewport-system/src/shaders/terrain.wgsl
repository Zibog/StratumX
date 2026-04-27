// Terrain Shader - Textured rendering with albedo sampling

struct CameraUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(2) @binding(0)
var terrain_albedo: texture_2d<f32>;

@group(2) @binding(1)
var terrain_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
    output.normal = vec3<f32>(0.0, 1.0, 0.0); // Up-facing normal for terrain
    output.color = input.color;
    output.uv = input.uv * 16.0; // UV scale for terrain texture repeat
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional lighting
    let light_dir = normalize(vec3<f32>(0.5, 0.7, 0.3));
    let ambient = 0.3;
    let diffuse = max(dot(normalize(input.normal), light_dir), 0.0);
    let lighting = ambient + diffuse * 0.7;

    // Sample terrain albedo texture
    let tex = textureSample(terrain_albedo, terrain_sampler, input.uv).rgb;
    
    // Combine texture with vertex color (height-based)
    let base = tex * input.color;
    
    return vec4<f32>(base * lighting, 1.0);
}
