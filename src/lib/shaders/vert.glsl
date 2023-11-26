
uniform mat4 embedMatrix;

varying vec2 vUv;
out vec3 vColor;
varying vec3 vClipPosition;

// FUNCTIONS

vec3 cylindricalToCartesian(vec3 pos) {
    float theta = pos.x;
    float r = pos.z;
    float y = pos.y;
    return vec3(- r * cos(theta * PI * 2.0), y, r * sin(theta * PI * 2.0));
}

// all shaders have a main function
void main() {

    vec3 vertPosition = position;
    vec3 vertEmbed = (embedMatrix * vec4(vertPosition, 1.0)).xyz;
    // REPLACE
    vec4 mvPosition = modelViewMatrix * vec4(vertPosition, 1.0);
    // #if NUM_CLIPPING_PLANES > 0

    vClipPosition = - mvPosition.xyz;
        // mvPosition *= 2;

    // #endif
    gl_Position = projectionMatrix * mvPosition;
    vColor = vertEmbed;
    vUv = uv;
}
