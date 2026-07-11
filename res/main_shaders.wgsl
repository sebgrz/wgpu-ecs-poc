const WIDTH: f32 = 800;
const HEIGHT: f32 = 600;
const SPRITE_TILE_SIZE: f32 = 0.125;

fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> mat4x4f {
 return mat4x4f(
        2 / (right - left), 0, 0, 0,
        0, 2 / (top - bottom), 0, 0, 
        0, 0, 2 / (far - near), 0,
        (right + left) / (left - right), (top + bottom) / (bottom - top), near / (near - far), 1
    );
}

fn translation_with_identity_mat(t: vec3f) -> mat4x4<f32> {
    return mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        t.x, t.y, t.z, 1.0
    );
}

struct Sprite {
  rect: vec4<f32>,
  tex_clip: vec4<f32>
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
var<uniform> camera: vec3f;

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

    let ortho_mat = ortho(0, WIDTH, HEIGHT, 0, 0.1, 100);
    let camera_mat = translation_with_identity_mat(camera);
    let pos = points_arr[in_vertex_index];
    let tex_coords = tex_coords_arr[in_vertex_index];

    return VertexOutput(
        ortho_mat * camera_mat * vec4<f32>(
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
    return textureSample(texture, tex_sampler, in.tex_coords);
}
