#version 330 core
layout (location = 0) out vec4 frag_colour;


in vec2 texCoords;
in vec2 actualCoords;

uniform usampler2D bgMap;
uniform usampler3D tileMapBank0;
uniform usampler3D tileMapBank1;

uniform ivec2 scroll;
uniform bool tileBank;
uniform int drawCutoff;

uniform vec3 gbColour0;
uniform vec3 gbColour1;
uniform vec3 gbColour2;
uniform vec3 gbColour3;
uniform int bg_pal;

bool useBank1(uint tileId) {
	return tileId >= 128u;
}

void main()
{
	if (actualCoords.y < uint(drawCutoff)) {
		discard;
	}

	ivec2 actualCoordsInt = ivec2(actualCoords + scroll) % 256;
	ivec2 tile_position = ivec2(actualCoordsInt >> 3);
	ivec2 tile_coords = actualCoordsInt % 8;

	uint tileId = texelFetch(bgMap, tile_position, 0).r ;

	uint sampledLowByte = texelFetch(useBank1(tileId) ? tileMapBank1 : tileMapBank0, ivec3(0, tile_coords.y, tileId % 128u), 0).r % 256u;
	uint sampledHighByte = texelFetch(useBank1(tileId) ? tileMapBank1 : tileMapBank0, ivec3(1, tile_coords.y, tileId % 128u), 0).r % 256u;

	bool isLightPixel = ((int(sampledLowByte) << tile_coords.x) & 128) != 0;
	bool isDarkPixel = ((int(sampledHighByte) << tile_coords.x) & 128) != 0;

	int colour = (isLightPixel ? 1 : 0) + (isDarkPixel ? 2 : 0);
	vec3 outColour;

	switch ((bg_pal >> colour * 2) & 3) {
		case 0:
			outColour = gbColour0;
			break;
		case 1:
			outColour = gbColour1;
			break;
		case 2:
			outColour = gbColour2;
			break;
		case 3:
			outColour = gbColour3;
			break;
	}

	frag_colour = vec4(outColour, 1.0);
}
