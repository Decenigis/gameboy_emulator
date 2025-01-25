#version 330 core
layout (location = 0) out vec4 frag_colour;


in vec2 texCoords;
in vec2 actualCoords;

uniform usampler2D bgMap;
uniform usampler3D tileMapBank0;
uniform usampler3D tileMapBank1;

uniform ivec2 screenPos;
uniform bool tileBank;


bool useBank1(uint tileId) {
	return tileId >= 128u;
}

void main()
{
	ivec2 actualCoordsInt = ivec2(actualCoords + screenPos) % 256;
	ivec2 tile_position = ivec2(actualCoordsInt >> 3);
	ivec2 tile_coords = actualCoordsInt % 8;

	uint tileId = texelFetch(bgMap, tile_position, 0).r ;

	uint sampledLowByte = texelFetch(useBank1(tileId) ? tileMapBank1 : tileMapBank0, ivec3(0, tile_coords.y, tileId % 128u), 0).r % 256u;
	uint sampledHighByte = texelFetch(useBank1(tileId) ? tileMapBank1 : tileMapBank0, ivec3(1, tile_coords.y, tileId % 128u), 0).r % 256u;

	bool isLightPixel = ((int(sampledLowByte) << tile_coords.x) & 128) != 0;
	bool isDarkPixel = ((int(sampledHighByte) << tile_coords.x) & 128) != 0;

	int colour = (isLightPixel ? 0 : 1) + (isDarkPixel ? 0 : 2);

	frag_colour = vec4(vec3(colour / 3.0), 1.0);
}
