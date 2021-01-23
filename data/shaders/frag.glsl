varying highp vec3 v_lighting;

void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);

    highp vec3 rgb = vec3(0.75, 0.75, 0.75) * v_lighting;
    gl_FragColor = vec4(rgb.xyz, 1.0);
}