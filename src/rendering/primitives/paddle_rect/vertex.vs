#version 410 core

in vec2 c0;
in vec2 c1;
in vec2 c2;
in vec2 c3;
in float height;
in vec3 pos;
in mat2 rot;
in vec4 color;
in int detail_level;
in vec2 anim_pos;
in mat2 anim_rot;

out vec2 c0_vs;
out vec2 c1_vs;
out vec2 c2_vs;
out vec2 c3_vs;
out float height_vs;
out vec3 pos_vs;
out mat2 rot_vs;
out vec4 color_vs;
out int detail_level_vs;
out vec2 anim_pos_vs;
out mat2 anim_rot_vs;

void main()
{
  c0_vs = c0;
  c1_vs = c1;
  c2_vs = c2;
  c3_vs = c3;
  height_vs = height;
  pos_vs = pos;
  rot_vs = rot;
  color_vs = color;
  detail_level_vs = detail_level;
  anim_pos_vs = anim_pos;
  anim_rot_vs = anim_rot;
}
