
#define PI 3.141592653

in vec3 pos;

uniform float tag;

layout (location = 0) out vec4 color;

vec3 xyz2hsv(vec3 pos)
{
    float v = pos.y;
    float s = clamp(length(pos.xz), 0.0, 1.0);
    float hp = atan(pos.z, pos.x)/PI/2.0;
    float h = clamp(hp + step(hp, 0.0), 0.0, 1.0);
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

vec3 linear_srgb_to_srgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

float max3 (vec3 v) {
  return max (max (v.x, v.y), v.z);
}

float min3 (vec3 v) {
  return min (min (v.x, v.y), v.z);
}

// vec3 oklab_to_ciexyz(vec3 oklab) {
//     mat3 m1 = mat3(
//         0.8189330101, 0.3618667424, -0.1288597137,
//         0.0329845436, 0.9293118715, 0.0361456387,
//         0.0482003018, 0.2643662691, 0.6338517070
//     );
//     mat3 m2 = mat3(
//         0.
//     );
// }

vec3 mark_out_of_gamut(vec3 rgb) {
    float mn = min3(rgb);
    float mx = max3(rgb);
    float below = 1.0 - smoothstep(-0.01, 0.0, mn);
    float above = smoothstep(1.0, 1.01, mx);
    float outside = max(below, above);
    return mix(rgb, vec3(0.5, 0.0, 0.5), outside);
}

vec3 oklab_to_linear_srgb(vec3 lab) {
    vec3 lms = vec3(lab.x + 0.3963377774 * lab.y + 0.2158037573 * lab.z,
                    lab.x - 0.1055613458 * lab.y - 0.0638541728 * lab.z,
                    lab.x - 0.0894841775 * lab.y - 1.2914855480 * lab.z);
    lms = pow(lms, vec3(3.0));
    vec3 rgb = vec3(4.0767416621 * lms.x - 3.3077115913 * lms.y + 0.2309699292 * lms.z,
                -1.2684380046 * lms.x + 2.6097574011 * lms.y - 0.3413193965 * lms.z,
                -0.0041960863 * lms.x - 0.7034186147 * lms.y + 1.7076147010 * lms.z);
    return rgb;
}

vec3 linear_srgb_to_oklab(vec3 rgb) {
    vec3 lms = vec3(0.4121656120 * rgb.x + 0.5362752080 * rgb.y + 0.0514575653 * rgb.z,
                    0.2118591070 * rgb.x + 0.6807189584 * rgb.y + 0.1074065790 * rgb.z,
                    0.0883097947 * rgb.x + 0.2818474174 * rgb.y + 0.6302613616 * rgb.z);
    lms = pow(lms, vec3(1.0/3.0, 1.0/3.0, 1.0/3.0));
    vec3 oklab = vec3(0.2104542553 * lms.x + 0.7936177850 * lms.y - 0.0040720468 * lms.z,
                    1.9779984951 * lms.x - 2.4285922050 * lms.y + 0.4505937099 * lms.z,
                    0.0259040371 * lms.x + 0.7827717662 * lms.y - 0.8086757660 * lms.z);
    // vec3 outside = 1.0-step(vec3(1.0, 1.0, 1.0), oklab);
    return oklab;
}

vec3 oklab_to_srgb(vec3 oklab) {
    return linear_srgb_to_srgb(mark_out_of_gamut(oklab_to_linear_srgb(oklab)));
}

// vec3 oklab_to_linear_srgb(vec3 oklab) {
//     mat3 m1 = mat3(1.0, 0.3963377774, 0.2158037573,
//                    1.0, -0.1055613458, -0.0638541728,
//                    1.0, -0.0894841775, -1.2914855480);
//     mat3 m2 = mat3(4.0767416621, -3.3077115913, 0.2309699292,
//                    -1.2684380046, 2.6097574011, -0.3413193965,
//                    -0.0041960863, -0.7034186147, 1.7076147010);
//     vec3 lms = m1 * oklab;
//     lms = pow(lms, vec3(3.0, 3.0, 3.0));
//     return m2 * lms;
// }


void main(){
    // REPLACE
}