
uniform float tag;
uniform sampler2D tDiffuse;

varying vec2 vUv;
in vec3 vColor;
in vec4 mvPosition;

#ifdef USE_CLIP_PLANE
varying vec3 vClipPosition;
uniform vec4 clipPlane;
#endif

// FUNCTIONS

float simple_srgbf(float x) {
    return pow(x, 1.0/2.2);
}

float srgbf(float x) {
    return x > 0.0034 ? pow(x, 1.0/2.4)*1.055-0.055 : x*12.92;
}

vec3 srgb(vec3 rgb) {
    return vec3(srgbf(rgb.x), srgbf(rgb.y), srgbf(rgb.z));
}
    
void main() {

    #ifdef USE_CLIP_PLANE
        if ( dot( vClipPosition, clipPlane.xyz ) > clipPlane.w ) discard;
    #endif
    
    vec4 fragColor = vec4(vColor, 1.0);
    // REPLACE
    gl_FragColor = fragColor;
}