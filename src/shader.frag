#version 420

layout(location = 0) in vec4 col;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = col;
}