#[test]
fn test_load() {
    use live2d_cubism_core_sys::core as live2d;
    

    let core = live2d::CubismCore::default();
    let moc = core.moc_from_bytes(include_bytes!("C:\\Program Files (x86)\\Steam\\steamapps\\common\\VTube Studio\\VTube Studio_Data\\StreamingAssets\\Live2DModels\\夢婆\\夢婆\\孟婆.moc3")).unwrap();
    let model = live2d::Model::from_moc(&moc);
    // let r#static = model.get_static();
    // let drawables = r#static.drawables();
    // let parts = r#static.parts();
    // let parameters = r#static.parameters();
    // println!("Drawable Count: {}", drawables.len());
    // for (i, drawable) in drawables.iter().enumerate() {
    //     println!("Drawable {}: {:?}", i, drawable);
    // }

    // println!("Part Count: {}", parts.len());
    // for (i, part) in parts.iter().enumerate() {
    //     println!("Part {}: {:?}", i, part);
    // }

    // println!("Parameter Count: {}", parameters.len());
    // for (i, parameter) in parameters.iter().enumerate() {
    //     println!("Parameter {}: {:?}", i, parameter);
    // }

    let dynamic = model.read_dynamic();
    dynamic.drawable_draw_orders();
}