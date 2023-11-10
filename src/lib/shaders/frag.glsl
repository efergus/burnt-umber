
in vec3 vColor;

// FUNCTIONS

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
    vec3 fragColor = vColor;
    // REPLACE
    gl_FragColor = vec4(fragColor, 1.0);
}