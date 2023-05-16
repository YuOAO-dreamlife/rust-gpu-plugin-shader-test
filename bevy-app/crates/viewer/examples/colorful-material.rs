use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::AsBindGroup};

use bevy_rust_gpu::{
    prelude::{RustGpu, RustGpuMaterialPlugin, RustGpuPlugin},
    EntryPoint, RustGpuMaterial,
};

// 匯入測試benchmark用的函式庫
use bevy::app::AppExit;
use std::time::Duration;

fn main() {
    let mut app = App::default();

    // 添加背景顏色
    app.insert_resource(ClearColor(Color::hex("071f3c").unwrap()));

    // 添加 Bevy的預設插件
    app.add_plugins(DefaultPlugins);

    // 添加 Rust-GPU插件
    app.add_plugin(RustGpuPlugin::default());

    // 設置 `RustGpu<ColorfulMaterial>`
    app.add_plugin(RustGpuMaterialPlugin::<ColorfulMaterial>::default());

    // 輸出 渲染進入的入口點檔案
    RustGpu::<ColorfulMaterial>::export_to(ENTRY_POINTS_PATH);

    // 開始執行時的動作
    app.add_startup_system(setup);

    // 為了方便測試 benchmark，在此設定經過1秒後會自動關閉 Bevy軟體
    app.add_system(exit_system);

    // 執行
    app.run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RustGpu<ColorfulMaterial>>>,
) {
    // 視角相機
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 載入渲染檔案
    let shader = asset_server.load(SHADER_PATH);

    // 生成方塊 + 使用移動方塊系統
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(RustGpu {
            vertex_shader: Some(shader.clone()),
            fragment_shader: Some(shader),
            ..default()
        }),
        ..default()
    });

    // benchmark測試用程式碼
    commands.spawn((FuseTime {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Once),
    },));
}

/// 和 SPIR-V渲染相關的檔案路徑
const SHADER_PATH: &'static str = "rust-gpu\\shader.rust-gpu.msgpack";

const ENTRY_POINTS_PATH: &'static str = "crates/viewer/entry_points.json";

/// 表示"點"(vertex)的渲染入口點名稱為`vertex_warp`
pub enum VertexWarp {}

impl EntryPoint for VertexWarp {
    const NAME: &'static str = "vertex_warp_colorful";
}

/// 表示"面"(fragment)的渲染入口點名稱為`fragment_normal`
pub enum FragmentNormal {}

impl EntryPoint for FragmentNormal {
    const NAME: &'static str = "fragment_normal_colorful";
}

/// Rust-GPU Material 與`VertexWarp` 和`FragmentNormal`做連結
#[derive(Debug, Default, Copy, Clone, AsBindGroup, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct ColorfulMaterial {}

impl Material for ColorfulMaterial {}

impl RustGpuMaterial for ColorfulMaterial {
    type Vertex = VertexWarp;
    type Fragment = FragmentNormal;
}

// 以下為 benchmark測試用程式碼
#[derive(Component)]
struct FuseTime {
    timer: Timer,
}

fn exit_system(mut exit: EventWriter<AppExit>, mut q: Query<&mut FuseTime>, time: Res<Time>) {
    for mut fuse_timer in q.iter_mut() {
        // 時間倒數。
        fuse_timer.timer.tick(time.delta());

        // 若時間到則關閉視窗。
        if fuse_timer.timer.finished() {
            exit.send(AppExit);
        }
    }
}
