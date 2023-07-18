pub const contents : &str = r#"
[gd_scene load_steps=3 format=3 uid="uid://bkc882742yiep"]

[ext_resource type="PackedScene" uid="uid://c070ppdurayj1" path="res://afps/fps_map_ghost_city.glb" id="1_f1lps"]

[sub_resource type="BoxMesh" id="BoxMesh_mnxtt"]

[node name="afps_placement_Test" type="Node3D"]

[node name="SKIP-ghost_city" parent="." instance=ExtResource("1_f1lps")]
transform = Transform3D(1.5, 0, 0, 0, 1.5, 0, 0, 0, 1.5, 0, 0, 0)

[node name="cube" type="MeshInstance3D" parent="."]
transform = Transform3D(11.0178, 0, 3.28098, 0, 11.4959, 0, -3.28098, 0, 11.0178, -21.5907, 0, 8.12672)
mesh = SubResource("BoxMesh_mnxtt")

[node name="laserbox" type="MeshInstance3D" parent="."]
transform = Transform3D(0.991558, 0, 0.129661, 0, 1, 0, -0.129661, 0, 0.991558, 33.1915, 0.215621, 3.10906)
mesh = SubResource("BoxMesh_mnxtt")

[node name="(beam)" type="MeshInstance3D" parent="laserbox"]
transform = Transform3D(0.04, 0, 1.14441e-06, 0, 0.04, 0, -2.28882e-09, 0, 20, 0, 0, 10)
mesh = SubResource("BoxMesh_mnxtt")
skeleton = NodePath("../..")

[node name="laserbox2" type="MeshInstance3D" parent="."]
transform = Transform3D(0.392241, 0, 0.919863, 0, 1, 0, -0.919863, 0, 0.392241, 14.8155, 0.215621, 13.767)
mesh = SubResource("BoxMesh_mnxtt")

[node name="(beam)" type="MeshInstance3D" parent="laserbox2"]
transform = Transform3D(0.04, 0, 1.14441e-06, 0, 0.04, 0, -2.28882e-09, 0, 20, 0, 0, 10)
mesh = SubResource("BoxMesh_mnxtt")
skeleton = NodePath("../..")

"#;