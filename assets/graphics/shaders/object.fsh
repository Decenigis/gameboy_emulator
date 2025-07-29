#version 330 core
layout (location = 0) out vec4 frag_colour;


in vec2 texCoords;
in vec2 actualCoords;

uniform usampler3D tileMapBank0;
uniform usampler3D tileMapBank1;

uniform ivec2 drawCutoff;

uniform vec3 gbColour0;
uniform vec3 gbColour1;
uniform vec3 gbColour2;
uniform vec3 gbColour3;

uniform int objPal0;
uniform int tileId;

uniform ivec2 objectPosition;
uniform int verticalFlip;
uniform int horizontalFlip;
uniform int dmgPalette;

bool useBank1(uint tileId) {
	return tileId >= 128u;
}

void main()
{
	if (int(actualCoords.x + objectPosition.x - 8) < drawCutoff.x || int(actualCoords.y + objectPosition.y - 16) != drawCutoff.y) {
		discard;
	}

	uint tileIdUint = uint(tileId);

	if (actualCoords.y > 8.0) {
		tileIdUint++;
	}

	ivec2 tile_coords = ivec2(actualCoords) % 8;

	if (horizontalFlip != 0) {
		tile_coords.x = 7 - tile_coords.x;
	}
	if (verticalFlip != 0) {
		tile_coords.y = 7 - tile_coords.y;
	}

	uint sampledLowByte, sampledHighByte;

	if (useBank1(tileIdUint))  {
		sampledLowByte = texelFetch(tileMapBank1, ivec3(0, tile_coords.y, tileIdUint % 128u), 0).r % 256u;
		sampledHighByte = texelFetch(tileMapBank1, ivec3(1, tile_coords.y, tileIdUint % 128u), 0).r % 256u;
	}
	else {
		sampledLowByte = texelFetch(tileMapBank0, ivec3(0, tile_coords.y, tileIdUint % 128u), 0).r % 256u;
		sampledHighByte = texelFetch(tileMapBank0, ivec3(1, tile_coords.y, tileIdUint % 128u), 0).r % 256u;
	}

	bool isLightPixel = ((int(sampledLowByte) << tile_coords.x) & 128) != 0;
	bool isDarkPixel = ((int(sampledHighByte) << tile_coords.x) & 128) != 0;

	int colour = (isLightPixel ? 1 : 0) + (isDarkPixel ? 2 : 0);

	if (colour == 0) {
		discard;
	}

	vec3 outColour;

	switch ((objPal0 >> colour * 2) & 3) {
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
