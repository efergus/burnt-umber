
uniform mat4 embedMatrix;

varying vec2 vUv;
out vec3 vColor;
varying vec3 vClipPosition;

// FUNCTIONS

// all shaders have a main function
void main() {

    vec3 vertPosition = position;
    vec3 vertEmbed = (embedMatrix * vec4(vertPosition, 1.0)).xyz;
    // REPLACE
    vec4 mvPosition = modelViewMatrix * vec4(vertPosition, 1.0);
    vClipPosition = vertPosition;
    gl_Position = projectionMatrix * mvPosition;
    vColor = vertEmbed;
    vUv = uv;
}
