use super::CameraProjection;
use bevy_app::prelude::{EventReader, Events};
use bevy_ecs::{Component, Local, Query, Res};
use bevy_math::Mat4;
use bevy_property::Properties;
use bevy_window::{WindowCreated, WindowId, WindowResized, Windows};

#[derive(Default, Debug, Properties)]
pub struct Camera {
    pub projection_matrix: Mat4,
    pub name: Option<String>,
    #[property(ignore)]
    pub window: WindowId,
    #[property(ignore)]
    pub depth_calculation: DepthCalculation,
}

#[derive(Debug)]
pub enum DepthCalculation {
    Distance,
    ZDifference,
}

impl Default for DepthCalculation {
    fn default() -> Self {
        DepthCalculation::Distance
    }
}

#[derive(Default)]
pub struct CameraSystemState {
    window_resized_event_reader: EventReader<WindowResized>,
    window_created_event_reader: EventReader<WindowCreated>,
}

pub fn camera_system<T: CameraProjection + Component>(
    mut state: Local<CameraSystemState>,
    window_resized_events: Res<Events<WindowResized>>,
    window_created_events: Res<Events<WindowCreated>>,
    windows: Res<Windows>,
    query: Query<(&mut Camera, &mut T)>,
) {
    let mut changed_window_ids = Vec::new();
    // handle resize events. latest events are handled first because we only want to resize each window once
    for event in state
        .window_resized_event_reader
        .iter(&window_resized_events)
        .rev()
    {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    // handle resize events. latest events are handled first because we only want to resize each window once
    for event in state
        .window_created_event_reader
        .iter(&window_created_events)
        .rev()
    {
        if changed_window_ids.contains(&event.id) {
            continue;
        }

        changed_window_ids.push(event.id);
    }

    for (mut camera, mut camera_projection) in &mut query.iter() {
        if let Some(window) = windows.get(camera.window) {
            if changed_window_ids.contains(&window.id) {
                camera_projection.update(window.width as usize, window.height as usize);
                camera.projection_matrix = camera_projection.get_projection_matrix();
                camera.depth_calculation = camera_projection.depth_calculation();
            }
        }
    }
}
