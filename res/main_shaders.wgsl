// struct size should be multiplicity of 16 bytes
struct Sprite {
  rect: vec4<f32>,
  tex_clip: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@group(1)
@binding(0)
var<uniform> sprites: array<Sprite, 1024>;

@group(2)
@binding(0)
var<uniform> camera: mat4x4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32, @builtin(instance_index) instance: u32) -> VertexOutput {
    let sprite = sprites[instance];

    let points_arr = array(
      vec2f(0, sprite.rect.w),
      vec2f(0, 0),
      vec2f(sprite.rect.z, 0),
      vec2f(sprite.rect.z, 0),
      vec2f(sprite.rect.z, sprite.rect.w),
      vec2f(0, sprite.rect.w),
    );
    let tex_coords_arr = array(
      vec2f(sprite.tex_clip.x, sprite.tex_clip.w),
      vec2f(sprite.tex_clip.x, sprite.tex_clip.y),
      vec2f(sprite.tex_clip.z, sprite.tex_clip.y),
      vec2f(sprite.tex_clip.z, sprite.tex_clip.y),
      vec2f(sprite.tex_clip.z, sprite.tex_clip.w),
      vec2f(sprite.tex_clip.x, sprite.tex_clip.w),
    );

    let pos = points_arr[in_vertex_index];
    let tex_coords = tex_coords_arr[in_vertex_index];

    return VertexOutput(
        camera * vec4<f32>(
          sprite.rect.x + pos.x, 
          sprite.rect.y + pos.y, 
          1.0, 1.0),
        vec2f(   
          tex_coords.x,
          tex_coords.y
        )
    );
}

@group(0)
@binding(0)
var texture: texture_2d<f32>;

@group(0)
@binding(1)
var tex_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, tex_sampler, in.tex_coords);
    let diff = distance(color.rgb, vec3<f32>(0.0, 0.0, 0.0));
    if (diff < 0.05) {
      discard;
    }
    return color;
}
