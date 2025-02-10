use bevy::{prelude::*, scene, tasks::IoTaskPool};

use std::{fs::File, io::Write, path::Path};

use crate::physics::{PhysicsMaterial, PhysicsTransform, PhysicsVelocity};

macro_rules! scenebuilder_builder {
    ( $world:expr, $( $T:ty ),* ) => {
        DynamicSceneBuilder::from_world($world)
        .deny_all()
        $(.allow_component::<$T>())*
    };
    ( $world:expr ) => {
        DynamicSceneBuilder::from_world($world)
    }
}

pub(crate) fn save_state(save_name: In<String>, world: &mut World) {
    let scene = scenebuilder_builder!(
        world,
        PhysicsTransform,
        Transform,
        PhysicsMaterial,
        PhysicsVelocity
    )
    .extract_entities(world.iter_entities().map(|entity| entity.id()))
    .extract_resources()
    .build();

    let registry = world.resource::<AppTypeRegistry>();
    let registry = registry.read();
    let serialized_scene = scene
        .serialize(&registry)
        .inspect_err(|e| {
            dbg!(e);
        })
        .expect("Failed To Serialize Gamestate");

    if save_name.len() <= 0 {
        error!("Cant save state without name");
        return;
    }

    IoTaskPool::get()
        .spawn(async move {
            write_savefile(
                &Path::new(&format!("./saves/{}", save_name.0)),
                serialized_scene.as_bytes(),
            );
        })
        .detach();
}

pub(crate) fn load_state(mut commands: Commands) {
    todo!();
}

fn write_savefile(path: &Path, buf: &[u8]) {
    #[cfg(not(target_family = "wasm"))]
    // Write the scene RON data to file
    File::create(path)
        .and_then(|mut file| file.write(buf))
        .expect("Error while writing scene to file");

    #[cfg(target_family = "wasm")]
    warn!("Saving is not supported in webassembly");
}
