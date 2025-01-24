#version 330 core


layout(location = 0) in vec2 pos;
layout(location = 1) in vec2 tex;

uniform mat4 pv;

out vec2 texCoords;

void main()
{
    gl_Position = pv * vec4(pos.xy, 0.5, 1.0);//vec4(pos.xyz, 1.0);

    texCoords = tex;
}