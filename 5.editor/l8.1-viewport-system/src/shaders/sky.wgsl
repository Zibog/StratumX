// Sky shader - gradient sky with sun disk

struct SkyUniform {
    sun_direction: vec4<f32>,
    sun_color: vec4<f32>,
    sky_color_zenith: vec4<f32>,
    sky_color_horizon: vec4<f32>,
    time_of_day: f32,
};

@group(0) @binding(0)
var<uniform> sky: SkyUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4<f32>(input.position.xy, 0.999, 1.0); // Far plane
    output.uv = input.position.xy;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Convert screen space to sky direction
    let uv = input.uv;
    let sky_dir = normalize(vec3<f32>(uv.x, uv.y * 0.5 + 0.5, 1.0));
    
    // Gradient from horizon to zenith
    let horizon_factor = abs(sky_dir.y);
    let sky_color = mix(sky.sky_color_horizon, sky.sky_color_zenith, horizon_factor);
    
    // Sun disk
    let sun_dir = normalize(sky.sun_direction.xyz);
    let sun_dot = dot(sky_dir, sun_dir);
    let sun_disk = smoothstep(0.998, 0.999, sun_dot); // Sharp sun disk
    let sun_glow = smoothstep(0.95, 0.998, sun_dot) * 0.3; // Soft glow
    
    // Combine
    let final_color = mix(sky_color, sky.sun_color, sun_disk + sun_glow);
    
    return final_color;
}
