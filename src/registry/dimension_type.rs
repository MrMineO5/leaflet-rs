use leaflet_nbt::NBTTag;

pub struct DimensionType {
    pub coordinate_scale: f64,
    pub has_skylight: bool,
    pub has_ceiling: bool,
    pub ambient_light: f32,
    pub has_fixed_time: bool,
    pub monster_spawn_block_light_limit: i32,
    pub logical_height: i32,
    pub min_y: i32,
    pub height: i32,
    pub infiniburn: String,
    pub skybox: String,
    pub cardinal_light: String,
    // pub attributes: NBTCompound
    // pub timelines: Identifier | String | Vec<Identifier>
}

impl DimensionType {
    pub fn overworld() -> Self {
        DimensionType {
            coordinate_scale: 1f64,
            has_skylight: true,
            has_ceiling: false,
            ambient_light: 0f32,
            has_fixed_time: false,
            monster_spawn_block_light_limit: 15,
            logical_height: 256,
            min_y: 0,
            height: 256,
            infiniburn: "".into(),
            skybox: "none".into(),
            cardinal_light: "default".into(),
        }
    }

    pub fn to_nbt(&self) -> NBTTag {
        NBTTag::compound(None, vec![
            NBTTag::double(Some("coordinate_scale".into()), self.coordinate_scale),
            NBTTag::boolean(Some("has_skylight".into()), self.has_skylight),
            NBTTag::boolean(Some("has_ceiling".into()), self.has_ceiling),
            NBTTag::float(Some("ambient_light".into()), self.ambient_light),
            NBTTag::boolean(Some("has_fixed_time".into()), self.has_fixed_time),
            NBTTag::int(Some("monster_spawn_block_light_limit".into()), self.monster_spawn_block_light_limit),
            NBTTag::int(Some("logical_height".into()), self.logical_height),
            NBTTag::int(Some("min_y".into()), self.min_y),
            NBTTag::int(Some("height".into()), self.height),
            NBTTag::string(Some("infiniburn".into()), self.infiniburn.clone()),
            NBTTag::string(Some("skybox".into()), self.skybox.clone()),
            NBTTag::string(Some("cardinal_light".into()), self.cardinal_light.clone()),
            NBTTag::compound(Some("attributes".into()), vec![]),
            NBTTag::list(Some("timelines".into()), vec![])
        ])
    }
}
