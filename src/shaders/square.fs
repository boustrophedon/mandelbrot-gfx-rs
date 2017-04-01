#version 150


uniform float scale;
uniform vec2 position;

in vec2 uv;

out vec4 frag_color;

// we could really just do this in place
vec2 mandelbrot_iter(vec2 z, vec2 c) {
  vec2 znew = vec2(0, 0);
  znew.x = z.x*z.x - z.y*z.y + c.x;
  znew.y = 2*z.x*z.y + c.y;

  return znew;
}

void main() {

  vec2 c = uv;
  // correct for window aspect ratio
  c.x = (c.x)*(1280.0/720.0);
  c = scale*c + position;


  bool inside = true;

  vec2 z = vec2(0, 0);
  for (int iters = 0; iters < 100; iters++) {
    z = mandelbrot_iter(z, c);
    if (dot(z,z) > 4.0) {
      inside = false;
      break;
    }
  }

  if (inside) {
    frag_color = vec4(0.0, 0.0, 0.0, 1.0);
  }
  else {
    frag_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
}
