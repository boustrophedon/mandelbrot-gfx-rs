#version 150

in vec2 a_position;

out vec2 uv;

void main() {
  uv = a_position;
  gl_Position = vec4(a_position, 0.0, 1.0);
}
