
#define PI 3.141592653

in vec3 pos;
// in vec3 embed;

uniform float tag;

layout (location = 0) out vec4 color;

vec3 xyz2hsv(vec3 pos)
{
    float v = pos.y;
    float s = clamp(length(pos.xz), 0.0, 1.0);
    float h = clamp((atan(pos.x, pos.z)/PI + 1.0)/2.0, 0.0, 1.0);
    return vec3(h, s, v);
}

vec3 rgb2hsv(vec3 c)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 apx_srgb_from_linear_srgb(vec3 rgb) {
    vec3 g = vec3(2.2, 2.2, 2.2);
    vec3 ginv = 1.0 / g;
    return pow(rgb, ginv);
}

vec3 srgb_from_linear_srgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

void main(){
    // color = vec4(hsv2rgb(xyz2hsv(pos.xyz)), 1.0);
    // REPLACE
}