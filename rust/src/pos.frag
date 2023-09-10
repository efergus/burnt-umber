
#define PI 3.141592653

in vec3 pos;

layout (location = 0) out vec4 color;

void main(){
    color = vec4(pos, 1.0f);
}