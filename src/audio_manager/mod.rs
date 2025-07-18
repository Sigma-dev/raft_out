use bevy::audio::{PlaybackMode, Volume};
use bevy::platform::collections::HashMap;
use bevy::{ecs::system::*, prelude::*};

#[derive(Clone)]
pub struct PlayAudio2D {
    pub path: String,
    pub volume_mult: f32,
    pub one_shot: bool,
    pub spatial_settings: Option<SpatialSettings>,
}

impl PlayAudio2D {
    pub fn new_once(path: impl Into<String>) -> PlayAudio2D {
        PlayAudio2D {
            path: path.into(),
            volume_mult: 1.,
            one_shot: true,
            spatial_settings: None,
        }
    }

    pub fn new_repeating(path: impl Into<String>) -> PlayAudio2D {
        PlayAudio2D {
            path: path.into(),
            volume_mult: 1.,
            one_shot: false,
            spatial_settings: None,
        }
    }

    pub fn with_volume(&self, volume_mult: f32) -> PlayAudio2D {
        let mut new = self.clone();
        new.volume_mult = volume_mult;
        new
    }
}

#[derive(Component)]
pub struct AudioFollower {
    pub followed: Entity,
}

#[derive(Component)]
pub struct TogglableAudio {
    pub tag: String,
}

#[derive(Clone, Copy)]
pub struct SpatialSettings {
    pub position: Vec2,
    pub follow: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct AudioManagerResource {
    audio_handles: HashMap<String, Handle<AudioSource>>,
    volume_mult: f32,
}

impl AudioManagerResource {
    pub fn new(volume_mult: f32) -> AudioManagerResource {
        AudioManagerResource {
            audio_handles: HashMap::default(),
            volume_mult,
        }
    }
}

#[derive(SystemParam)]
pub struct AudioManager<'w, 's> {
    #[doc(hidden)]
    commands: Commands<'w, 's>,
    #[doc(hidden)]
    asset_server: Res<'w, AssetServer>,
    #[doc(hidden)]
    resource: ResMut<'w, AudioManagerResource>,
    #[doc(hidden)]
    audios: Query<'w, 's, (&'static mut AudioSink, &'static mut TogglableAudio)>,
}

impl<'w, 's> AudioManager<'w, 's> {
    pub fn get_audio_handle(&mut self, path: &String) -> Handle<AudioSource> {
        if let Some(audio) = self.resource.audio_handles.get(path) {
            return audio.clone();
        }
        let handle = self.asset_server.load(path.clone());
        self.resource
            .audio_handles
            .insert(path.clone(), handle.clone());
        handle
    }

    pub fn play_sound(&mut self, sound: PlayAudio2D) {
        let playback_settings = PlaybackSettings {
            mode: if sound.one_shot {
                PlaybackMode::Despawn
            } else {
                PlaybackMode::Loop
            },
            volume: Volume::Linear(1. * sound.volume_mult * self.resource.volume_mult),
            ..default()
        };
        let source = self.get_audio_handle(&sound.path);
        if let Some(settings) = &sound.spatial_settings {
            let mut e = self.commands.spawn((
                AudioPlayer::new(source),
                playback_settings.with_spatial(true),
                Transform::from_translation(settings.position.extend(0.)),
            ));
            if let Some(followed) = settings.follow {
                e.insert(AudioFollower { followed });
            }
        } else {
            self.commands.spawn((
                AudioPlayer::new(source),
                playback_settings.with_spatial(false),
            ));
        }
    }

    pub fn toggle_audio(
        &mut self,
        path: impl Into<String>,
        toggle: bool,
        volume_mult: Option<f32>,
    ) {
        let path = path.into();
        let mut found = false;
        for (sink, togglable) in self.audios.iter_mut() {
            if togglable.tag == *path {
                found = true;
                if toggle {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
        }
        if !found && toggle {
            let playback_settings = PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::Linear(1. * volume_mult.unwrap_or(1.) * self.resource.volume_mult),
                spatial: false,
                ..default()
            };
            let source = self.get_audio_handle(&path);
            self.commands.spawn((
                AudioPlayer::new(source),
                playback_settings,
                TogglableAudio {
                    tag: path.to_string(),
                },
            ));
        }
    }

    pub fn toggle_audio_off(&mut self, path: impl Into<String>) {
        self.toggle_audio(path, false, None);
    }
}

pub struct AudioManagerPlugin {
    pub volume_mult: f32,
}

impl Default for AudioManagerPlugin {
    fn default() -> Self {
        Self { volume_mult: 1. }
    }
}

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_followers)
            .insert_resource(AudioManagerResource::new(self.volume_mult));
    }
}

fn handle_followers(
    mut followers_query: Query<(&mut Transform, &AudioFollower)>,
    transforms_query: Query<&Transform, Without<AudioFollower>>,
) {
    for (mut follower_transform, follower) in followers_query.iter_mut() {
        follower_transform.translation =
            transforms_query.get(follower.followed).unwrap().translation;
    }
}
