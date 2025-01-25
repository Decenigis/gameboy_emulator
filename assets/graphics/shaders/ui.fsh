#version 330 core
layout (location = 0) out vec4 frag_colour;


in vec2 texCoords;
in vec2 actualCoords;

uniform sampler2D texture0;
uniform ivec2 screenPos;

void main()
{
	ivec2 actualCoordsInt = ivec2(actualCoords + screenPos) % 256;
	ivec2 tile = ivec2(actualCoordsInt / 8);

	vec4 sampledColor = texture(texture0, texCoords);

	if (sampledColor.a < 0.005)
		discard;

	frag_colour = vec4(float(tile.x) / 32.0, float(tile.y) / 32.0, 0.0, 1.0 );
}