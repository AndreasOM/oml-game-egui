#version 330 core
out vec4 FragColor;
in vec2 oTexCoords;
in vec4 oColor;
// varying ?

uniform sampler2D texture0;

void main()
{
	FragColor = oColor * texture( texture0, oTexCoords ) + vec4( oTexCoords.y, 0, 0, 1);
	//FragColor = oColor * vec4( oTexCoords, 1, 2);
} 
