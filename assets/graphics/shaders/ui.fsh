#version 330 core
layout (location = 0) out vec4 frag_colour;


in vec2 texCoords;

uniform sampler2D texture0;

void main()
{
	vec4 sampledColor = texture(texture0, texCoords);

	if (sampledColor.a < 0.005)
		discard;

	frag_colour = sampledColor;
}