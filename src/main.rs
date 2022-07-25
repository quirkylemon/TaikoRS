use bevy::prelude::*;
use std::io::Read;
use std::fs;
use bevy::window::WindowFocused;

const SIZE: f32 = 96.0;
const NOTE_HEIGHT: f32 = 180.0;
const HIT_ZONE_X: f32 = -640.0;
const ASSETS_ROOT: &str = "./TaikoRS/";

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

//markers and events
#[derive(Component)]
struct Note;

#[derive(Component)]
struct HitZone;

#[derive(Component)]
struct PauseMenuButton;

#[derive(Component)]
struct ResumeButton;

#[derive(Component)]
struct OptionsButton;

#[derive(Component)]
struct ExitButton;

struct SongStart;
struct SongEnd;

// bevy components
#[derive(Clone, Copy, Component, Debug, PartialEq)]
enum NoteTypeEnum {
    DonSmall,
    KaSmall,
    DonLarge,
    KaLarge,
    Drumroll(f32),
    Ballon(f32),
    None,
}


#[derive(Bundle)]
struct NoteExpanded {
    x: Transform,
    note_type: NoteTypeEnum,
}

// non bevy structs
struct NotesInSong {
    notes: Vec<NoteOptimized>,
}

#[derive(Clone, Copy)]
struct NoteOptimized {
    // this number will be divided by 3 to find the actual position this allows for really close notes like this ())), look at boku wa synth to see what I mean
    x: u32,
    // Also drumroll or balloon I just couldnt come up with a good name. false is drumroll, true is balloon
    don_or_ka: bool,
    // This will be ignored if the type is drumroll or ballon. false is small, true is large
    size: bool,
    // false is Don/Ka, true is drumroll/balloon(decided by don_or_ka variable)
    type_of_note: bool,
    // if the note type is drumroll or ballon this will decide how long it is
    length_of_special: u16,
}

// later when a GUI is added this will be set based on where you click
struct SongPath {
    path: String,
}

// this will probably have more modifiers later 
struct Modifiers {
    speed: f32,
}

struct HitWindow {
    good: f32,
    ok: f32,
    bad: f32,
}

#[derive(Debug)]
struct Score {
    score: u16,
    goods: u32,
    oks: u32,
    bads: u32,
}

#[derive(Debug, Clone, Copy)]
enum EnumInput {
    Don,
    Ka,
    None,
}


#[derive(Debug)]
struct InputLeftSide {
    input: EnumInput,
}

struct InputRightSide {
    input: EnumInput,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum MenuState {
    MainMenu,
    OptionsMenu,
    SongSelectMenu,
    EditorMenu,
    Playing,
    None,
}

struct PreviousMenuState {
    state: MenuState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PlayState {
    EditorTestMode,
    PlayMode,
    TrainingMode,
    Paused,
}

struct PreviousPlayState {
    state: PlayState,
}

// Writing this was one of the worst times I've had while coding, it isnt even good or complex code :(
fn load_notes_from_file(mut commands: Commands, mut notes: ResMut<NotesInSong>, path: Res<SongPath>, asset_server: Res<AssetServer>) {
    
    fn read_file_to_string(default_path: &str) -> String {
        let mut string = String::new();
        let mut _path = match std::env::args().len() {
            0 => "no path".to_string(),
            1 => "no path".to_string(),
            _ => std::env::args().nth(1).unwrap(),
        };
        match _path.as_str() {
            "no path" => {_path = default_path.to_string() + "/Map.txt"},
            _ => {_path = _path + "/Map.txt"}
            
        }
        match std::fs::File::open(_path.to_string()) {
            Ok(_) => {
                    let mut file = std::fs::File::open(_path).unwrap();
                    file.read_to_string(&mut string).unwrap();
                },
            Err(e) => {log_error(format!("error opening file: error {}", e)); string = "error".to_string();}
        }      
        return string  
    }

    let contents = read_file_to_string(&path.path);
    if contents != "error".to_string() {
        let split = contents.split('{');
        let mut raw_notes: Vec<&str> = split.collect();

        raw_notes.retain(|value| *value != "\n");
        raw_notes.retain(|value| *value != "");
                
        let mut _metadata = raw_notes[0];
        raw_notes.remove(0);

        let mut invalid_notes: Vec<usize> = vec![];
        for i in 0..raw_notes.len() {
            let mut note_info: Vec<&str> = raw_notes[i].split(',').collect();
            note_info.retain(|value| *value != "\n");

            let _x: u32 = match note_info[0].parse::<u32>() {
                Ok(_) => {note_info[0].parse::<u32>().unwrap()},
                Err(_) => {
                    if !invalid_notes.contains(&i) {
                        invalid_notes.push(i);
                    } 
                    0
                }
            }; 

            let _don_or_ka: bool = match note_info[1] {
                "0" => false,
                "1" => true,
                _ => {
                    if !invalid_notes.contains(&i) {
                        invalid_notes.push(i);
                    }
                    false
                }
            };

            let _size: bool = match note_info[2] {
                "0" => false,
                "1" => true,
                _ => {
                    if !invalid_notes.contains(&i) {
                        invalid_notes.push(i);
                    }
                    false
                }
            };

            let _type_of_note: bool = match note_info[3] {
                "0" => false,
                "1" => true,
                _ => {
                    if !invalid_notes.contains(&i) {
                        invalid_notes.push(i);
                    }
                    false
                }
            };

            let _length_of_special: u16 = match note_info[4].parse::<u16>() {
                Ok(_) => note_info[4].parse::<u16>().unwrap(),
                Err(_) => {
                    if !invalid_notes.contains(&i) {
                        invalid_notes.push(i);
                    }
                    0
                }
            };

            if invalid_notes.len() == 0 {
                notes.notes.push(NoteOptimized { x: _x, don_or_ka: _don_or_ka, size: _size, type_of_note: _type_of_note, length_of_special:  _length_of_special})
            } 
        }
        invalid_notes = invalid_notes.into_iter().map(|i| i + 1).collect();
        #[cfg(debug_assertions)] {
            if invalid_notes.len() > 0 {
                log_error(invalid_notes)
            }  
        }

        fn expand(note: NoteOptimized) -> (Transform, NoteTypeEnum, String) {
            let _note_type = match (note.don_or_ka, note.size, note.type_of_note) {
                (false, false, false) => NoteTypeEnum::KaSmall,
                (true, false, false) => NoteTypeEnum::DonSmall,
                (false, true, false) => NoteTypeEnum::KaLarge,
                (true, true, false) => NoteTypeEnum::DonLarge,
                (false, true | false, true) => NoteTypeEnum::Drumroll(note.length_of_special as f32 / 3.0),
                (true, true | false, true) => NoteTypeEnum::Ballon(note.length_of_special as f32)
            };

            let _sprite = match _note_type {
                NoteTypeEnum::KaSmall => "Sprites/Ka.png",
                NoteTypeEnum::DonSmall => "Sprites/Don.png",
                NoteTypeEnum::DonLarge => "Sprites/Don.png",
                NoteTypeEnum::KaLarge => "Sprites/Ka.png",
                _ => "Sprites/Don.png"
            };

            let _position = Transform::from_xyz(note.x as f32 / 3.0, 0.0, 0.0);
            (_position, _note_type, _sprite.to_string())
        }

        if notes.notes.len() > 0 {
            for i in 0..notes.notes.len() {
                #[allow(unused_must_use)] {
                commands.spawn()
                    .insert(Note)
                    .insert_bundle(NoteExpanded{
                        x: expand(notes.notes[i]).0,
                        note_type: expand(notes.notes[i]).1
                    })
                    .insert_bundle(
                        SpriteBundle {
                            texture: asset_server.load(&expand(notes.notes[i]).2),
                            transform: Transform::from_xyz(notes.notes[i].x as f32 / 3.0 * SIZE, NOTE_HEIGHT as f32, 0.0),
                            visibility: Visibility { is_visible: false },
                            sprite: Sprite { 
                                custom_size: match expand(notes.notes[i]).1 {
                                    NoteTypeEnum::KaSmall => Some(Vec2::new(SIZE, SIZE)),
                                    NoteTypeEnum::DonSmall => Some(Vec2::new(SIZE, SIZE)),
                                    NoteTypeEnum::DonLarge => Some(Vec2::new(SIZE * 1.25, SIZE * 1.25)),
                                    NoteTypeEnum::KaLarge => Some(Vec2::new(SIZE * 1.25, SIZE * 1.25)),
                                    _ => Some(Vec2::new(SIZE, SIZE))
                                }, ..default()
                            },
                            ..default()
                        }
                    )
                    .id();
                }
            }
        }
    } else {
        log_error("error opening map please check if file exists");
    }
    
}

fn pause_menu_button_system(mut interaction_query: Query<(
    &Interaction,
    &mut UiColor,), 
    (Changed<Interaction>, With<Button>)>,
    )
    {
    for (interaction, mut color) in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }

            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }

            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


fn setup_pause_menu_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    // I am very sorry to anyone who has done UI/GUI before. This is my first time so I don't know if this is really wonky.
    // Please feel free to change it.
    commands.spawn_bundle(UiCameraBundle::default());
    // Resume button
    commands.spawn_bundle(ButtonBundle {
        style: Style { 
            size: Size::new(Val::Px(300.0), Val::Px(97.5)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect { left: Val::Percent(40.0), right: Val::Percent(0.0), top: Val::Percent(0.0), bottom: Val::Percent(65.0) },
            position_type: PositionType::Absolute,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .insert(PauseMenuButton)
    .insert(ResumeButton)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Resume", 
                TextStyle {
                    font: asset_server.load("Fonts/Modified-DFPKanteiryu-XB.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default()
            ),
            ..default()
        })
        .insert(PauseMenuButton)
        .insert(ResumeButton);
    });

    // Options button
    commands.spawn_bundle(ButtonBundle {
        style: Style { 
            size: Size::new(Val::Px(300.0), Val::Px(97.5)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect { left: Val::Percent(40.0), right: Val::Percent(0.0), top: Val::Percent(0.0), bottom: Val::Percent(50.0) },
            position_type: PositionType::Absolute,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .insert(PauseMenuButton)
    .insert(OptionsButton)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Options", 
                TextStyle {
                    font: asset_server.load("Fonts/Modified-DFPKanteiryu-XB.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default()
            ),
            ..default()
        })
        .insert(PauseMenuButton)
        .insert(OptionsButton);
    });

    // Exit button
    commands.spawn_bundle(ButtonBundle {
        style: Style { 
            size: Size::new(Val::Px(300.0), Val::Px(97.5)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position: Rect { left: Val::Percent(40.0), right: Val::Percent(0.0), top: Val::Percent(0.0), bottom: Val::Percent(35.0) },
            position_type: PositionType::Absolute,
            ..default()
        },
        color: NORMAL_BUTTON.into(),
        ..default()
    })
    .insert(PauseMenuButton)
    .insert(ExitButton)
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text::with_section(
                "Exit", 
                TextStyle {
                    font: asset_server.load("Fonts/Modified-DFPKanteiryu-XB.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default()
            ),
            ..default()
        })
        .insert(PauseMenuButton)
        .insert(ExitButton);
    });
}

fn pause_menu_buttons(
    key_input: Res<Input<KeyCode>>,
    mut play_state: ResMut<State<PlayState>>, 
    mut previous_play_state: ResMut<PreviousPlayState>
) {
   if key_input.any_just_pressed([KeyCode::Escape]) == true {
        if *play_state.current() != PlayState::Paused {
            match play_state.overwrite_set(PlayState::Paused) {
                Ok(_) => {
                    previous_play_state.state = *play_state.current();
                    play_state.overwrite_set(PlayState::Paused).unwrap();
                },
                Err(e) => {log_error(e);},
            }
        } else if *play_state.current() == PlayState::Paused {
            match play_state.overwrite_set(previous_play_state.state) {
                Ok(_) => {
                    play_state.overwrite_set(previous_play_state.state).unwrap();
                    previous_play_state.state = *play_state.current();
                },
                Err(e) => log_error(e),
            }
        }
   }
}

fn pause_menu_visibilty(mut button_query: Query<&mut Visibility, With<PauseMenuButton>>, play_state: ResMut<State<PlayState>>) {
    for mut visibility in button_query.iter_mut() {
        //text doesnt turn invisible
        if *play_state.current() != PlayState::Paused {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = true;
        }
    } 
}

fn log_error<T: std::fmt::Debug>(error: T) {
    println!("{:?}", error)
}

fn update_notes(
    mut commands: Commands,
    mut query: Query<(&NoteTypeEnum, Entity, &mut Transform, &mut Visibility), With<Note>>,
    input_left: Res<InputLeftSide>,
    input_right: Res<InputRightSide>,
    hit_window: Res<HitWindow>,
    mut score: ResMut<Score>,
    modifiers: Res<Modifiers>,
    timer: Res<Time>, 
    window: Res<WindowDescriptor>
) {
    fn input_to_note_type(input_left: EnumInput, input_right: EnumInput) -> NoteTypeEnum {
        match (input_left, input_right) {
            (EnumInput::Don, EnumInput::Don) => NoteTypeEnum::DonLarge,
            (EnumInput::Ka, EnumInput::Ka) => NoteTypeEnum::KaLarge,
            (EnumInput::Don, EnumInput::None) => NoteTypeEnum::DonSmall,
            (EnumInput::None, EnumInput::Don) => NoteTypeEnum::DonSmall,
            (EnumInput::Ka, EnumInput::None) => NoteTypeEnum::KaSmall,
            (EnumInput::None, EnumInput::Ka) => NoteTypeEnum::KaSmall,
            (EnumInput::Don, EnumInput::Ka) => NoteTypeEnum::DonSmall,
            (EnumInput::Ka, EnumInput::Don) => NoteTypeEnum::KaSmall,
            (EnumInput::None, EnumInput::None) => NoteTypeEnum::None,
            
        }
    }
        for (note_type, ent, mut transform, mut visible) in query.iter_mut() {
            transform.translation.x -= 100.0 * timer.delta_seconds() * modifiers.speed as f32;
            if transform.translation.x < -window.width / 2.0 {
                score.bads += 1;
                commands.entity(ent).despawn();
            } else if transform.translation.x > window.width + 200.0 {
                visible.is_visible = false;
            } else {
                visible.is_visible = true;
            }
        
            if ((HIT_ZONE_X - hit_window.good)..(HIT_ZONE_X + hit_window.good)).contains(&(transform.translation.x)) && input_to_note_type(input_left.input, input_right.input) == *note_type {
                score.goods += 1;
                // placeholder value
                score.score += 100;
                commands.entity(ent).despawn();
            } else if ((HIT_ZONE_X - hit_window.ok)..(HIT_ZONE_X + hit_window.ok)).contains(&(transform.translation.x)) && input_to_note_type(input_left.input, input_right.input) == *note_type {
                score.oks += 1;
                // placeholder value
                score.score += 50;
                commands.entity(ent).despawn();
            } else if ((HIT_ZONE_X - hit_window.bad)..(HIT_ZONE_X + hit_window.bad)).contains(&(transform.translation.x)) && input_to_note_type(input_left.input, input_right.input) == *note_type{
                score.bads += 1;
                commands.entity(ent).despawn();
            }
        }
    }   

fn input_detection(
    key_input: Res<Input<KeyCode>>, 
    mut left_input: ResMut<InputLeftSide>, 
    mut right_input: ResMut<InputRightSide>
    
) {
    if key_input.just_pressed(KeyCode::D) {
        left_input.input = EnumInput::Ka;
    } else if key_input.just_pressed(KeyCode::F) {
        left_input.input = EnumInput::Don;
    }  else {
        left_input.input = EnumInput::None;
    }

    if key_input.just_pressed(KeyCode::K) {
        right_input.input = EnumInput::Ka;
    } else if key_input.just_pressed(KeyCode::J) {
        right_input.input = EnumInput::Don;
    } else {
        right_input.input = EnumInput::None;
    }
}


fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_song(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<WindowDescriptor>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("Sprites/HitZone.png"),
        transform: Transform::from_xyz(-window.width / 3.0, NOTE_HEIGHT as f32, 0.0),
        sprite: Sprite { custom_size: Some(Vec2::new(SIZE * 1.1, SIZE * 1.1)), ..default()},
        ..default()
    }).insert(HitZone); 
    
}

fn pause_when_out_of_focus(mut play_state: ResMut<State<PlayState>>, mut previous_state: ResMut<PreviousPlayState>, mut focused_evr: EventReader<WindowFocused>) {
    for i in focused_evr.iter() {
        if i.focused == false {
            match *play_state.current() {
                PlayState::Paused => {},
                _ => {previous_state.state = *play_state.current(); play_state.overwrite_set(PlayState::Paused).unwrap();},
            }
        } else {
            match *play_state.current() {
                PlayState::Paused => {
                    match play_state.overwrite_set(previous_state.state) {
                        Ok(_) => {play_state.overwrite_set(previous_state.state).unwrap(); previous_state.state = PlayState::Paused;},
                        Err(e) => log_error(e)
                    }
                },
                _ => {},
            }
        }
    }
} 

fn print_avaible_songs() {
    for folder in fs::read_dir(ASSETS_ROOT.to_string() + "Songs/").unwrap() {
        println!("{}", folder.as_ref().unwrap().path().display());
        for i in fs::read_dir(folder.as_ref().unwrap().path()).unwrap() {
            println!("   {}", i.unwrap().path().display());
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Taiko RS".to_string(),
            width: 1920.0,
            height: 1080.0,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(MenuState::Playing)
        .add_state(PlayState::Paused)
        .add_event::<WindowFocused>()
        .add_event::<SongStart>()
        .add_event::<SongEnd>()
        .insert_resource(NotesInSong{notes: vec![]})
        .insert_resource(SongPath{path: "TaikoRS/Songs/.Debug/TestSong/".to_string()})
        .insert_resource(Modifiers{speed: 1.0})
        .insert_resource(HitWindow{good: 15.0, ok: 30.0, bad: 45.0})
        .insert_resource(Score{score: 0, goods: 0, oks: 0, bads: 0})
        .insert_resource(InputLeftSide{input: EnumInput::None})
        .insert_resource(InputRightSide{input: EnumInput::None})
        .insert_resource(PreviousPlayState{state: PlayState::PlayMode})
        .insert_resource(PreviousMenuState{state: MenuState::Playing})
        .add_startup_system(setup_camera)
        .add_startup_system(load_notes_from_file)
        .add_startup_system(print_avaible_songs)
        .add_startup_system(setup_pause_menu_button)
        .add_system(pause_menu_button_system)
        .add_system(pause_menu_buttons)
        .add_system_set(
            SystemSet::on_enter(PlayState::PlayMode)
                .with_system(setup_song)
        )
        .add_system_set(
            SystemSet::on_update(PlayState::PlayMode)
                .with_system(input_detection.before(update_notes))
                .with_system(update_notes)
                .with_system(pause_when_out_of_focus)
        )
        .add_system_set(
            SystemSet::on_enter(PlayState::EditorTestMode)
                .with_system(setup_song)
        )
        .add_system_set(
            SystemSet::on_update(PlayState::EditorTestMode)
                .with_system(input_detection.before(update_notes))
                .with_system(update_notes)
                .with_system(pause_when_out_of_focus)
        )
        .add_system_set(
            SystemSet::on_enter(PlayState::TrainingMode)
                .with_system(setup_song)
        )
        .add_system_set(
            SystemSet::on_update(PlayState::TrainingMode)
                .with_system(input_detection.before(update_notes))
                .with_system(update_notes)
                .with_system(pause_when_out_of_focus)
        )
        .add_system_set(
            SystemSet::on_update(MenuState::Playing)
                .with_system(pause_menu_visibilty.after(pause_menu_buttons))
        )
        .run();
}
