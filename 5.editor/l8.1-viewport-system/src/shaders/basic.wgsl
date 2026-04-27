// Basic shader for terrain and sky rendering

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

struct SkyUniform {
    time_of_day: f32,      // 0.0 - 24.0 hours
    sun_elevation: f32,    // -90 to 90 degrees
    _padding1: f32,
    _padding2: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@group(1) @binding(0)
var<uniform> sky: SkyUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) world_pos: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.world_pos = model.position;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

// Sky gradient based on time of day
fn get_sky_color(view_dir: vec3<f32>) -> vec3<f32> {
    let elevation = view_dir.y;
    let time = sky.time_of_day;
    
    // Night (0-6, 18-24)
    var zenith_color: vec3<f32>;
    var horizon_color: vec3<f32>;
    
    if (time < 6.0 || time > 18.0) {
        // Night
        zenith_color = vec3<f32>(0.02, 0.02, 0.06);
        horizon_color = vec3<f32>(0.04, 0.04, 0.08);
    } else if (time < 8.0) {
        // Sunrise (6-8)
        let t = (time - 6.0) / 2.0;
        zenith_color = mix(vec3<f32>(0.02, 0.02, 0.06), vec3<f32>(0.3, 0.4, 0.6), t);
        horizon_color = mix(vec3<f32>(0.04, 0.04, 0.08), vec3<f32>(0.8, 0.5, 0.3), t);
    } else if (time < 16.0) {
        // Day (8-16)
        zenith_color = vec3<f32>(0.2, 0.4, 0.8);
        horizon_color = vec3<f32>(0.6, 0.7, 0.9);
    } else {
        // Sunset (16-18)
        let t = (time - 16.0) / 2.0;
        zenith_color = mix(vec3<f32>(0.2, 0.4, 0.8), vec3<f32>(0.02, 0.02, 0.06), t);
        horizon_color = mix(vec3<f32>(0.8, 0.5, 0.3), vec3<f32>(0.04, 0.04, 0.08), t);
    }
    
    // Gradient from horizon to zenith
    let t = clamp(elevation * 2.0 + 0.5, 0.0, 1.0);
    return mix(horizon_color, zenith_color, t);
}

// Main fragment shader - uses sky color for background
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Issue 9: Use sky calculation instead of just vertex color
    // Check if this is sky geometry (far from camera)
    let dist = length(in.world_pos);
    
    if (dist > 500.0) {
        // Sky/background - use sky shader
        let view_dir = normalize(in.world_pos);
        let sky_color = get_sky_color(view_dir);
        return vec4<f32>(sky_color, 1.0);
    } else {
        // Terrain/objects - use vertex color
        return vec4<f32>(in.color, 1.0);
    }
}

// Cloud fragment shader with alpha blending (Issue 9)
@fragment
fn fs_cloud(in: VertexOutput) -> @location(0) vec4<f32> {
    // Cloud color with alpha based on density
    let cloud_color = in.color;
    let alpha = cloud_color.r; // Use red channel as alpha
    return vec4<f32>(vec3<f32>(1.0, 1.0, 1.0), alpha * 0.7);
}
