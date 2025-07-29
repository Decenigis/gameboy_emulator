#version 330 core


layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 tex;

uniform mat4 pv;
uniform ivec2 objectPosition;

uniform int priority;

out vec2 texCoords;
out vec2 actualCoords;

void main()
{
    actualCoords = pos.xy;

    gl_Position = pv * vec4(pos.xy + vec2(objectPosition) - vec2(8, 16), 1.0, 1.0);//vec4(pos.xyz, 1.0);

    texCoords = tex;
    actualCoords = pos.xy;
}
