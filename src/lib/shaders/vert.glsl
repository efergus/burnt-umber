
uniform mat4 embedMatrix;

varying vec2 vUv;
out vec3 vColor;

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
    gl_Position = projectionMatrix * modelViewMatrix * vec4(vertPosition, 1.0);
    vColor = vertEmbed;
    vUv = uv;
}
