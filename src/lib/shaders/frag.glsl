#version 300 es
    
// fragment shaders don't have a default precision so we need
// to pick one. highp is a good default. It means "high precision"
precision highp float;

in vec3 v_position;
    
// we need to declare an output for the fragment shader
out vec4 outColor;

float simple_srgbf(float x) {
    return pow(x, 1.0/2.2);
}

float srgbf(float x) {
    return x > 0.0034 ? pow(x, 1.0/2.4)*1.055-0.055 : x*12.92;
}

vec3 srgb(vec3 rgb) {
    // rgb = vec3((rgb.xy + 1.0)/2.0, rgb.z);
    return vec3(srgbf(rgb.x), srgbf(rgb.y), srgbf(rgb.z));
}
    
void main() {
    outColor = vec4(srgb(v_position), 1.0);
}