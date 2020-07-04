use copy_dir;
use serde_json;
use std::fs;
use std::io::{self};
use std::path;
use uuid::Uuid;

pub struct AddonTemplate<'a> {
    pub addon_name: &'a str,
    pub author_name: &'a str,
    pub where_to_make: &'a path::Path,
    pub using_template_dir: &'a path::Path,
}

impl<'a> AddonTemplate<'a> {
    pub fn generate_addon<'b>(&'b self) -> Result<(), io::Error> {
        fs::create_dir(self.where_to_make.join(self.addon_name))?;
        self.generate_behavior_pack()?;
        self.generate_resource_pack()?;
        Ok(())
    }

    fn generate_behavior_pack<'b>(&'b self) -> Result<(), io::Error> {
        copy_dir::copy_dir(
            self.using_template_dir.join("templateBP"),
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}BP", self.addon_name)),
        )?;
        let manifest_json = self.behavior_pack_manifest();
        let manifest_writer = fs::File::create(
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}BP", self.addon_name))
                .join("manifest.json"),
        )?;
        serde_json::to_writer_pretty(manifest_writer, &manifest_json)?;
        Ok(())
    }
    fn generate_resource_pack<'b>(&'b self) -> Result<(), io::Error> {
        copy_dir::copy_dir(
            self.using_template_dir.join("templateRP"),
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}RP", self.addon_name)),
        )?;
        let manifest_json = self.resource_pack_manifest();
        let manifest_writer = fs::File::create(
            self.where_to_make
                .join(self.addon_name)
                .join(format!("{}RP", self.addon_name))
                .join("manifest.json"),
        )?;
        serde_json::to_writer_pretty(manifest_writer, &manifest_json)?;
        Ok(())
    }
    fn resource_pack_manifest<'b>(&'b self) -> serde_json::Value {
        let rp_uuid = Uuid::new_v4();
        let rp_modules_uuid = Uuid::new_v4();
        let rp_manifest = serde_json::json!(
            {
                "format_version": 2,
                "header": {
                    "description": format!("Created by {}", self.author_name),
                    "name": format!("{} Resource Pack", self.addon_name),
                    "uuid": &rp_uuid,
                    "version": [0, 0, 1],
                    "min_engine_version": [ 1, 14, 0 ]
                },
                "modules": [
                    {
                        "description": format!("{} Resource Pack", self.addon_name),
                        "type": "resources",
                        "uuid": &rp_modules_uuid,
                        "version": [0, 0, 1]
                    }
                ]
            }
        );
        rp_manifest
    }
    fn behavior_pack_manifest<'b>(&'b self) -> serde_json::Value {
        let bp_uuid = Uuid::new_v4();
        let bp_modules_uuid = Uuid::new_v4();
        let bp_manifest = serde_json::json!(
            {
                "format_version": 2,
                "header": {
                    "description": format!("Created by {}", self.author_name),
                    "name": format!("{} Behavior Pack", self.addon_name),
                    "uuid": &bp_uuid,
                    "version": [ 0, 0, 1 ],
                    "min_engine_version": [ 1, 14, 0 ]
                },
                "modules": [
                    {
                        "description": format!("{} Behavior Pack", self.addon_name),
                        "type": "data",
                        "uuid": &bp_modules_uuid,
                        "version": [0, 0, 1]
                    }
                ]
            }
        );
        bp_manifest
    }
}
