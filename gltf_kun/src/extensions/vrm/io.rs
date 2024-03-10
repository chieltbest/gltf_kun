use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vrm {
    #[serde(rename = "exporterVersion")]
    pub exporter_version: Option<String>,
    #[serde(rename = "specVersion")]
    pub spec_version: Option<String>,
    pub meta: Option<Meta>,
    pub humanoid: Humanoid,
    #[serde(rename = "firstPerson")]
    pub first_person: Option<FirstPerson>,
    #[serde(rename = "blendShapeMaster")]
    pub blend_shape_master: Option<BlendShapeMaster>,
    #[serde(rename = "secondaryAnimation")]
    pub secondary_animation: Option<SecondaryAnimation>,
    #[serde(rename = "materialProperties")]
    pub material_properties: Option<Vec<MaterialProperty>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Meta {
    pub title: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    #[serde(rename = "contactInformation")]
    pub contact_information: Option<String>,
    pub reference: Option<String>,
    pub texture: Option<u32>,
    #[serde(rename = "allowedUserName")]
    pub allowed_user_name: Option<String>,
    #[serde(rename = "violentUssageName")]
    pub violent_usage_name: Option<String>,
    #[serde(rename = "sexualUssageName")]
    pub sexual_usage_name: Option<String>,
    #[serde(rename = "commercialUssageName")]
    pub commercial_usage_name: Option<String>,
    #[serde(rename = "otherPermissionUrl")]
    pub other_permission_url: Option<String>,
    #[serde(rename = "licenseName")]
    pub license_name: Option<String>,
    #[serde(rename = "otherLicenseUrl")]
    pub other_license_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Humanoid {
    #[serde(rename = "humanBones")]
    pub human_bones: Option<Vec<Bone>>,
    #[serde(rename = "armStretch")]
    pub arm_stretch: Option<f32>,
    #[serde(rename = "legStretch")]
    pub leg_stretch: Option<f32>,
    #[serde(rename = "upperArmTwist")]
    pub upper_arm_twist: Option<f32>,
    #[serde(rename = "lowerArmTwist")]
    pub lower_arm_twist: Option<f32>,
    #[serde(rename = "upperLegTwist")]
    pub upper_leg_twist: Option<f32>,
    #[serde(rename = "lowerLegTwist")]
    pub lower_leg_twist: Option<f32>,
    #[serde(rename = "feetSpacing")]
    pub feet_spacing: Option<f32>,
    #[serde(rename = "hasTranslationDoF")]
    pub has_translation_dof: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bone {
    pub bone: Option<String>,
    pub node: Option<u32>,
    #[serde(rename = "useDefaultValues")]
    pub use_default_values: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FirstPerson {
    #[serde(rename = "firstPersonBone")]
    pub first_person_bone: Option<u32>,
    #[serde(rename = "firstPersonBoneOffset")]
    pub first_person_bone_offset: Option<Vec3>,
    #[serde(rename = "meshAnnotations")]
    pub mesh_annotations: Option<Vec<MeshAnnotation>>,
    #[serde(rename = "lookAtTypeName")]
    pub look_at_type_name: Option<String>,
    #[serde(rename = "lookAtHorizontalInner")]
    pub look_at_horizontal_inner: Option<LookAtCurve>,
    #[serde(rename = "lookAtHorizontalOuter")]
    pub look_at_horizontal_outer: Option<LookAtCurve>,
    #[serde(rename = "lookAtVerticalDown")]
    pub look_at_vertical_down: Option<LookAtCurve>,
    #[serde(rename = "lookAtVerticalUp")]
    pub look_at_vertical_up: Option<LookAtCurve>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MeshAnnotation {
    pub mesh: Option<u32>,
    #[serde(rename = "firstPersonFlag")]
    pub first_person_flag: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct LookAtCurve {
    pub curve: Option<[f32; 8]>,
    #[serde(rename = "xRange")]
    pub x_range: Option<f32>,
    #[serde(rename = "yRange")]
    pub y_range: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BlendShapeMaster {
    #[serde(rename = "blendShapeGroups")]
    pub blend_shape_groups: Option<Vec<BlendShapeGroup>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BlendShapeGroup {
    pub name: Option<String>,
    #[serde(rename = "presetName")]
    pub preset_name: Option<String>,
    pub binds: Option<Vec<Bind>>,
    #[serde(rename = "materialValues")]
    pub material_values: Option<Vec<MaterialBind>>,
    #[serde(rename = "isBinary")]
    pub is_binary: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MaterialBind {
    #[serde(rename = "materialName")]
    pub material_name: Option<String>,
    #[serde(rename = "propertyName")]
    pub property_name: Option<String>,
    #[serde(rename = "targetValue")]
    pub target_value: Option<Vec<f32>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Bind {
    pub mesh: Option<u32>,
    pub index: Option<u32>,
    pub weight: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SecondaryAnimation {
    #[serde(rename = "boneGroups")]
    pub bone_groups: Option<Vec<BoneGroup>>,
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<ColliderGroup>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BoneGroup {
    pub comment: Option<String>,
    pub stiffiness: Option<f32>,
    #[serde(rename = "gravityPower")]
    pub gravity_power: Option<f32>,
    #[serde(rename = "gravityDir")]
    pub gravity_dir: Option<Vec3>,
    #[serde(rename = "dragForce")]
    pub drag_force: Option<f32>,
    pub center: Option<f32>,
    #[serde(rename = "hitRadius")]
    pub hit_radius: Option<f32>,
    pub bones: Option<Vec<u32>>,
    #[serde(rename = "colliderGroups")]
    pub collider_groups: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColliderGroup {
    pub node: Option<u32>,
    pub colliders: Option<Vec<Collider>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Collider {
    pub offset: Option<Vec3>,
    pub radius: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MaterialProperty {
    pub name: Option<String>,
    #[serde(rename = "renderQueue")]
    pub render_queue: Option<i32>,
    pub shader: Option<String>,
    #[serde(rename = "floatProperties")]
    pub float: Option<FloatProperties>,
    #[serde(rename = "vectorProperties")]
    pub vector: Option<VectorProperties>,
    #[serde(rename = "textureProperties")]
    pub texture: Option<TextureProperties>,
    #[serde(rename = "keywordMap")]
    pub keyword_map: Option<KeywordMap>,
    #[serde(rename = "tagMap")]
    pub tag_map: Option<TagMap>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FloatProperties {
    #[serde(rename = "_ShadeShift")]
    pub shade_shift: Option<f32>,
    #[serde(rename = "_ShadeToony")]
    pub shade_toony: Option<f32>,
    #[serde(rename = "_Cutoff")]
    pub cutoff: Option<f32>,
    #[serde(rename = "_IndirectLightIntensity")]
    pub indirect_light_insensity: Option<f32>,
    #[serde(rename = "_OutlineWidth")]
    pub outline_width: Option<f32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TextureProperties {
    #[serde(rename = "_MainTex")]
    pub main_tex: Option<u32>,
    #[serde(rename = "_ShadeTexture")]
    pub shade_texture: Option<u32>,
    #[serde(rename = "_BumpMap")]
    pub bump_map: Option<u32>,
    #[serde(rename = "_SphereAdd")]
    pub sphere_add: Option<u32>,
    #[serde(rename = "_EmissionMap")]
    pub emission_map: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VectorProperties {
    #[serde(rename = "_Color")]
    pub color: Option<[f32; 4]>,
    #[serde(rename = "_ShadeColor")]
    pub shade_color: Option<[f32; 4]>,
    #[serde(rename = "_OutlineColor")]
    pub outline_color: Option<[f32; 4]>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TagMap {
    #[serde(rename = "RenderType")]
    pub render_type: Option<RenderType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum RenderType {
    Opaque,
    Transparent,
    TransparentCutout,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeywordMap {
    #[serde(rename = "_ALPHABLEND_ON")]
    pub alpha_blend: Option<bool>,
    #[serde(rename = "_ALPHATEST_ON")]
    pub alpha_test: Option<bool>,
    #[serde(rename = "_NORMALMAP")]
    pub normal_map: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_COLOR_FIXED")]
    pub outline_color_fixed: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_COLOR_MIXED")]
    pub outline_color_mixed: Option<bool>,
    #[serde(rename = "MTOON_OUTLINE_WIDTH_WORLD")]
    pub outline_width_world: Option<bool>,
}
