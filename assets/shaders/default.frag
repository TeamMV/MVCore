#version 450

precision highp float;

layout (location = 0) in vec4 fColor;
layout (location = 1) in vec2 fTexCoords;
layout (location = 2) in float fTexID;
layout (location = 3) in vec4 fCanvasCoords;//(x, y, width, height)
layout (location = 4) in vec2 fCanvasData;//([0 = sq, 1 = tri, 2 = circ], radius)
layout (location = 5) in vec2 fRes;

out vec4 outColor;

uniform sampler2D TEX_SAMPLER[MAX_TEXTURE_IMAGE_UNITS];

float sq(float x) {
    return x * x;
}

void main() {
    float type = fCanvasData.x;
    float r = fCanvasData.y;
    if (fCanvasCoords.x > gl_FragCoord.x || fCanvasCoords.x + fCanvasCoords.z < gl_FragCoord.x || fCanvasCoords.y > gl_FragCoord.y || fCanvasCoords.y + fCanvasCoords.w < gl_FragCoord.y) {
        discard;
    }
    else if (type != 0 && r > 0) {
        if (type == 1) {
            if (gl_FragCoord.x - fCanvasCoords.x < gl_FragCoord.y - (fCanvasCoords.y + fCanvasCoords.w - r)) {
                discard;
            }
            if ((fCanvasCoords.x + fCanvasCoords.z) - gl_FragCoord.x < gl_FragCoord.y - (fCanvasCoords.y + fCanvasCoords.w - r)) {
                discard;
            }
            if (gl_FragCoord.x - fCanvasCoords.x < (fCanvasCoords.y + r) - gl_FragCoord.y) {
                discard;
            }
            if (gl_FragCoord.x - (fCanvasCoords.x + fCanvasCoords.z - r) > gl_FragCoord.y - fCanvasCoords.y) {
                discard;
            }
        }
        else if (type == 2) {
            if (gl_FragCoord.x < fCanvasCoords.x + r && gl_FragCoord.y > fCanvasCoords.y + fCanvasCoords.w - r && sq((fCanvasCoords.x + r) - gl_FragCoord.x) + sq(gl_FragCoord.y - (fCanvasCoords.y + fCanvasCoords.w - r)) > sq(r)) {
                discard;
            }
            if (gl_FragCoord.x > fCanvasCoords.x + fCanvasCoords.z - r && gl_FragCoord.y > fCanvasCoords.y + fCanvasCoords.w - r && sq(gl_FragCoord.x - (fCanvasCoords.x + fCanvasCoords.z - r)) + sq(gl_FragCoord.y - (fCanvasCoords.y + fCanvasCoords.w - r)) > sq(r)) {
                discard;
            }
            if (gl_FragCoord.x < fCanvasCoords.x + r && gl_FragCoord.y < fCanvasCoords.y + r && sq((fCanvasCoords.x + r) - gl_FragCoord.x) + sq((fCanvasCoords.y + r) - gl_FragCoord.y) > sq(r)) {
                discard;
            }
            if (gl_FragCoord.x > fCanvasCoords.x + fCanvasCoords.z - r && gl_FragCoord.y < fCanvasCoords.y + r && sq(gl_FragCoord.x - (fCanvasCoords.x + fCanvasCoords.z - r)) + sq((fCanvasCoords.y + r) - gl_FragCoord.y) > sq(r)) {
                discard;
            }
        }
    }
    if (fTexID > 0) {
        vec4 c = texture(TEX_SAMPLER[int(fTexID) - 1], fTexCoords);
        outColor = c;

        if (fColor.w > 0.0) {
            outColor = vec4(fColor.x, fColor.y, fColor.z, c.w);
        } else {
            outColor = c;
        }
    }
    else {
        outColor = fColor;
    }
}