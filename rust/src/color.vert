in vec3 position;
uniform mat4 model;
uniform mat4 view;
out vec3 pos;

void main() {
    vec4 model_pos = model * vec4(position, 1.0);
    gl_Position = view * model_pos;
    pos = model_pos.xyz;
}