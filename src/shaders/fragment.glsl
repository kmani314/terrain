#version 120
uniform vec3 position;
uniform float[16] thresholds;
uniform int num_thresh;
vec3 color;
uniform vec3      light_position;
uniform sampler2D tex;
varying vec2      tex_coord;
varying vec3      ws_normal;
varying vec3      ws_position;
void main() {
  vec3 L = normalize(light_position - ws_position);
  vec3 E = normalize(-ws_position);

  // Height Based Colors:
	float height = ws_position.y/50;	
	if (height < 0.4) {
		color = vec3(0.211, 0.404, 0.850);
	} else if (height < 0.435) {
			color = vec3(0.75, 0.80, 0.255);
	} else if (height < 0.5) {
		color = vec3(0.256, 0.6, 0.07);
	} else if (height < 0.53) {
		color = vec3(0.18, 0.5, 0.07);
	} else if (height < 0.65) {
		color = vec3(0.2, 0.11, 0.10);
	} else if (height < 0.8) {
		color = vec3(0.1, 0.05, 0.05);
	} else {
		color = vec3(1.0, 1.0, 1.0);
	}
  //calculate Ambient Term:
  vec4 Iamb = vec4(color, 1.0);

  //calculate Diffuse Term:
  vec4 Idiff1 = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(ws_normal,L), 0.0);
  Idiff1 = clamp(Idiff1, 0.0, 1.0);

  // double sided lighting:
  vec4 Idiff2 = vec4(1.0, 1.0, 1.0, 1.0) * max(dot(-ws_normal,L), 0.0);
  Idiff2 = clamp(Idiff2, 0.0, 1.0);

  vec4 tex_color = texture2D(tex, tex_coord);
  gl_FragColor   = tex_color * (Iamb + (Idiff1 + Idiff2) / 2) / 2;
}
