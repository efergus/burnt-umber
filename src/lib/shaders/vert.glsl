
uniform mat4 embedMatrix;

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
    vec3 vertPosition = (embedMatrix * vec4(position, 1.0)).xyz;
    vec3 vertColor = vertPosition;
    // REPLACE
    gl_Position = projectionMatrix * modelViewMatrix * vec4(vertPosition, 1.0);
    vColor = vertColor;
}
