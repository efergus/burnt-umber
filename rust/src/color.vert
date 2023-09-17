in vec3 position;
in vec3 embed;
uniform mat4 model;
uniform mat4 meta;
uniform mat4 view;
out vec3 pos;

void main() {
    vec4 p = vec4(position, 1.0);
    vec4 model_pos = model * p;
    gl_Position = view * model * p;
    vec4 meta_pos = meta * p;
    pos = meta_pos.xyz;
}