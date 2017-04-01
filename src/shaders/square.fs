#version 150

uniform float aspect;
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
  c.x = (c.x)*aspect;
  c = scale*c + position;


  bool inside = true;

  vec2 z = vec2(0, 0);
  float iters = 0.0;
  for (; iters < 300; iters+=1.0) {
    z = mandelbrot_iter(z, c);
    if (dot(z,z) > 4.0) {
      inside = false;
      break;
    }
  }

  if (inside) {
    frag_color = vec4(0.0, 0.0, 0.0, 1.0);
  }
  else if (iters < 3.0) {
    frag_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
  else {
    // fake stepped
    //frag_color = round(10*mix(vec4(1.0, 0.1, 0.5, 1.0), vec4(0.0, 0.0, 1.0, 1.0), iters/300))/10;
    frag_color = mix(vec4(1.0, 0.1, 0.5, 1.0), vec4(0.0, 0.0, 1.0, 1.0), iters/300);
  }
}
