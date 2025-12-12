#[path = "../src/schema/live2d/mod.rs"]
mod live2d;

use std::fs;
use std::path::PathBuf;
use serde_json::from_str;
use live2d::*;

const MODEL_DIR: &str = r"C:\Program Files (x86)\Steam\steamapps\common\VTube Studio\VTube Studio_Data\StreamingAssets\Live2DModels\Haru";

#[test]
fn test_model3_deserialization() {
    let path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
    
    let config: model::Config = from_str(&json).expect("Failed to deserialize model3.json");
    
    assert!(config.version >= 3);
    assert!(!config.file_references.moc.is_empty());
}

#[test]
fn test_physics3_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(physics_file) = model_config.file_references.physics {
        let path = PathBuf::from(MODEL_DIR).join(physics_file);
        let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
        let config: physics::Config = from_str(&json).expect("Failed to deserialize physics3.json");
        
        assert!(config.version >= 3);
        assert!(config.meta.physics_setting_count > 0);
    } else {
        println!("No physics file referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_userdata3_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(userdata_file) = model_config.file_references.user_data {
        let path = PathBuf::from(MODEL_DIR).join(userdata_file);
        let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
        let config: userdata::Config = from_str(&json).expect("Failed to deserialize userdata3.json");
        
        assert!(config.version >= 3);
    } else {
        println!("No userdata file referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_pose3_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(pose_file) = model_config.file_references.pose {
        let path = PathBuf::from(MODEL_DIR).join(pose_file);
        let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
        let config: pose::Config = from_str(&json).expect("Failed to deserialize pose3.json");
        
        match config.r#type {
            pose::PoseType::Live2DPose => {},
        }
    } else {
        println!("No pose file referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_display_info_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(display_info_file) = model_config.file_references.display_info {
        let path = PathBuf::from(MODEL_DIR).join(display_info_file);
        let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
        let config: display::Config = from_str(&json).expect("Failed to deserialize cdi3.json");
        
        assert!(config.version >= 3);
    } else {
        println!("No display info file referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_motionsync_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(motionsync_file) = model_config.file_references.motion_sync {
        let path = PathBuf::from(MODEL_DIR).join(motionsync_file);
        let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
        let config: motionsync::Config = from_str(&json).expect("Failed to deserialize motionsync3.json");
        
        assert!(config.version >= 3);
    } else {
        println!("No motion sync file referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_expression_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(expressions) = model_config.file_references.expressions {
        if let Some(first_exp) = expressions.first() {
            let path = PathBuf::from(MODEL_DIR).join(&first_exp.file);
            let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
            let config: expression::Config = from_str(&json).expect("Failed to deserialize exp3.json");
            
            match config.r#type {
                expression::ExpressionType::Live2DExpression => {},
            }
        }
    } else {
        println!("No expressions referenced in model3.json, skipping test.");
    }
}

#[test]
fn test_motion_deserialization() {
    let model_path = PathBuf::from(MODEL_DIR).join("Haru.model3.json");
    let model_json = fs::read_to_string(&model_path).expect("Failed to read model3.json");
    let model_config: model::Config = from_str(&model_json).expect("Failed to deserialize model3.json");

    if let Some(motions) = model_config.file_references.motions {
        for (_, motion_list) in motions {
            if let Some(first_motion) = motion_list.first() {
                let path = PathBuf::from(MODEL_DIR).join(&first_motion.file);
                let json = fs::read_to_string(&path).expect(&format!("Failed to read file: {:?}", path));
                let config: motion::Config = from_str(&json).expect("Failed to deserialize motion3.json");
                
                assert!(config.version >= 3);
                return;
            }
        }
        println!("No motion files found in any group, skipping test.");
    } else {
        println!("No motions referenced in model3.json, skipping test.");
    }
}
