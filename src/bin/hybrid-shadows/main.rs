use rendertoy::*;
use rtoy_rt::*;

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Constants {
    viewport_constants: ViewportConstants,
    light_dir: Vector4,
}

fn main() {
    let mut rtoy = Rendertoy::new();

    let tex_key = TextureKey {
        width: rtoy.width(),
        height: rtoy.height(),
        format: gl::RGBA32F,
    };

    //let scene_file = "assets/meshes/lighthouse.obj.gz";
    let scene_file = "assets/meshes/flying_trabant.obj.gz";

    let scene = load_obj_scene(scene_file.to_string());
    let bvh = build_gpu_bvh(scene);

    let mut camera = FirstPersonCamera::new(Point3::new(0.0, 100.0, 500.0));

    let viewport_constants_buf = init_dynamic!(upload_buffer(&0u32));

    let raster_uniforms = shader_uniforms!(
        "constants": viewport_constants_buf,
        "": make_raster_mesh(scene)
    );

    let gbuffer_tex = raster_tex(
        tex_key,
        make_raster_pipeline(vec![
            load_vs(asset!("shaders/raster_simple_vs.glsl")),
            load_ps(asset!("shaders/raster_gbuffer_ps.glsl")),
        ]),
        raster_uniforms,
    );

    let rt_uniforms = shader_uniforms!(
        "constants": viewport_constants_buf,
        "": bvh,
        "inputTex": gbuffer_tex,
    );

    let shadowed_tex = compute_tex(
        tex_key,
        load_cs(asset!("shaders/rt_hybrid_shadows.glsl")),
        rt_uniforms,
    );

    let mut light_angle = 0.0f32;

    rtoy.draw_forever(|frame_state| {
        camera.update(frame_state, 1.0 / 60.0);

        let viewport_constants =
            ViewportConstants::build(&camera, tex_key.width, tex_key.height).finish();

        light_angle += 0.01;

        redef_dynamic!(
            viewport_constants_buf,
            upload_buffer(Constants {
                viewport_constants,
                light_dir: Vector4::new(light_angle.cos(), 0.5, light_angle.sin(), 0.0)
            })
        );

        shadowed_tex
    });
}
