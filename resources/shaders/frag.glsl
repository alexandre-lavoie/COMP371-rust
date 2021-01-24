varying highp vec3 v_lighting;

void main() {
    highp vec3 rgb = vec3(0.75, 0.5, 0.75) * v_lighting;
    gl_FragColor = vec4(rgb.xyz, 1.0);
}