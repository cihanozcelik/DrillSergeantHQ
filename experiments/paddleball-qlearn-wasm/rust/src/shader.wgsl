struct Scene {
  paddle_x: f32,
  paddle_y: f32,
  paddle_w: f32,
  paddle_h: f32,
  ball_x: f32,
  ball_y: f32,
  ball_r: f32,
  aspect: f32,
  _pad0: f32,
  _pad1: f32,
  _pad2: f32,
}

@group(0) @binding(0)
var<uniform> scene: Scene;

struct VsOut {
  @builtin(position) pos: vec4<f32>,
  // NDC in [-1..1] used by fragment.
  @location(0) ndc: vec2<f32>,
}

// Fullscreen triangle.
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
  var positions = array<vec2<f32>, 3>(
    vec2<f32>(-1.0, -3.0),
    vec2<f32>( 3.0,  1.0),
    vec2<f32>(-1.0,  1.0),
  );

  let p = positions[vi];
  var out: VsOut;
  out.pos = vec4<f32>(p, 0.0, 1.0);
  out.ndc = p;
  return out;
}

fn rect_sdf_aspect(p: vec2<f32>, center: vec2<f32>, half: vec2<f32>, aspect: f32) -> f32 {
  // Signed distance to axis-aligned box.
  let d = abs(vec2<f32>((p.x - center.x) * aspect, p.y - center.y)) - vec2<f32>(half.x * aspect, half.y);
  return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
  // Convert from NDC [-1..1] to UV [0..1].
  let uv = in.ndc * 0.5 + vec2<f32>(0.5, 0.5);

  // Walls (static, visual only): top/left/right.
  let wall_t = 0.02;
  let wall_col = vec3<f32>(0.22, 0.24, 0.30);

  let left_center = vec2<f32>(wall_t * 0.5, 0.5);
  let left_half = vec2<f32>(wall_t * 0.5, 0.5);
  let d_left = rect_sdf_aspect(uv, left_center, left_half, scene.aspect);

  let right_center = vec2<f32>(1.0 - wall_t * 0.5, 0.5);
  let right_half = vec2<f32>(wall_t * 0.5, 0.5);
  let d_right = rect_sdf_aspect(uv, right_center, right_half, scene.aspect);

  let top_center = vec2<f32>(0.5, 1.0 - wall_t * 0.5);
  let top_half = vec2<f32>(0.5, wall_t * 0.5);
  let d_top = rect_sdf_aspect(uv, top_center, top_half, scene.aspect);

  // Paddle: center in UV space.
  let paddle_center = vec2<f32>(scene.paddle_x, scene.paddle_y);
  let paddle_half = vec2<f32>(scene.paddle_w * 0.5, scene.paddle_h * 0.5);
  let d_paddle = rect_sdf_aspect(uv, paddle_center, paddle_half, scene.aspect);

  // Ball: circle in UV space.
  let ball_center = vec2<f32>(scene.ball_x, scene.ball_y);
  let dv = vec2<f32>((uv.x - ball_center.x) * scene.aspect, uv.y - ball_center.y);
  let d_ball = length(dv) - scene.ball_r;

  // Colors.
  let bg = vec3<f32>(0.06, 0.07, 0.09);
  let paddle_col = vec3<f32>(0.30, 0.85, 0.75);
  let ball_col = vec3<f32>(0.95, 0.62, 0.22);

  var col = bg;
  if (d_left <= 0.0 || d_right <= 0.0 || d_top <= 0.0) {
    col = wall_col;
  }
  if (d_paddle <= 0.0) {
    col = paddle_col;
  }
  if (d_ball <= 0.0) {
    col = ball_col;
  }

  return vec4<f32>(col, 1.0);
}


