use bevy::prelude::*;
use std::io::Read;
use std::env;

const SIZE: f32 = 96.0;

const  NOTE_HEIGHT: u16 = 180;

//markers 
#[derive(Component)]
struct Note;

// bevy components
#[derive(Clone, Copy, Component, Debug)]
enum NoteTypeEnum {
    DonSmall,
    KaSmall,
    DonLarge,
    KaLarge,
    Drumroll(f32),
    Ballon(f32),
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

struct SongPath {
    path: String,
}

struct Modifiers {
    speed: f32,
}

#[derive(Debug)]
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

fn load_args(mut path: ResMut<SongPath>) {
    for arg in env::args() {
        match env::args().len() > 1 {
            false => {},
            true =>  path.path = arg,
        }
    }
    #[cfg(debug_assertions)]
    println!("{}", path.path);
}

fn load_notes_from_file(mut commands: Commands, mut notes: ResMut<NotesInSong>, path: Res<SongPath>, asset_server: Res<AssetServer>) {
    fn read_file_to_string(path: &str) -> String {
        let mut string = String::new();
        let _path = path.to_owned() + "Map.txt";
        match std::fs::File::open(_path.to_string()) {
            Ok(_) => {
                    let mut file = std::fs::File::open(_path).unwrap();
                    file.read_to_string(&mut string).unwrap();
                },
            Err(e) => {println!("error opening file: error {}", e); string = "error".to_string();}
        }      
        return string  
    }

    let contents = read_file_to_string(path.path.as_str());
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
            println!("{}", notes.notes.len());   
            println!("{:?}", invalid_notes);
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
                NoteTypeEnum::KaSmall => "Ka.png",
                NoteTypeEnum::DonSmall => "Don.png",
                NoteTypeEnum::DonLarge => "Don.png",
                NoteTypeEnum::KaLarge => "Ka.png",
                _ => "Don.png"
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
                            visibility: Visibility { is_visible: true },
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
        println!("error opening map please check if file exists");
    }
    
}


fn update_notes(
    mut commands: Commands,
    mut query: Query<(&NoteTypeEnum, Entity, &mut Transform, &mut Visibility), With<Note>>,
    input_left: Res<InputLeftSide>,
    input_right: Res<InputRightSide>,
    modifiers: Res<Modifiers>,
    timer: Res<Time>, 
    window: Res<WindowDescriptor>
) {
    
    //#[cfg(debug_assertions)] 
    //println!("{}", query.is_empty());

    for (_note_type, ent, mut transform, mut visible) in query.iter_mut() {
        transform.translation.x -= 100.0 * timer.delta_seconds() * modifiers.speed as f32;
        if transform.translation.x < -window.width / 2.0 {
            commands.entity(ent).despawn();
        } else if transform.translation.x > window.width + 200.0 {
            visible.is_visible = false;
        } else {
            visible.is_visible = true;
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
    #[cfg(debug_assertions)]
    println!("left: {:?}, right: {:?}", left_input.input, right_input.input);
}

fn print_notes(mut query: Query<(&mut Transform, &NoteTypeEnum), With<Note>>) {
    for (_transform, _note_type) in query.iter_mut() {
        //#[cfg(debug_assertions)]
        //println!("x: {}, type: {:?}", _transform.translation.x, _note_type)
    }
}


fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>, window: Res<WindowDescriptor>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("HitZone.png"),
        transform: Transform::from_xyz(-window.width / 3.0, NOTE_HEIGHT as f32, 0.0),
        sprite: Sprite { custom_size: Some(Vec2::new(SIZE * 1.1, SIZE * 1.1)), ..default()},
        ..default()
    });
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
        .insert_resource(NotesInSong{notes: vec![]})
        .insert_resource(SongPath{path: "TaikoRS/Songs/.Debug/TestSong/".to_string()})
        .insert_resource(Modifiers{speed: 1.0})
        .insert_resource(InputLeftSide{input: EnumInput::None})
        .insert_resource(InputRightSide{input: EnumInput::None})
        .add_startup_system(setup_camera)
        .add_startup_system(load_args.before(load_notes_from_file))
        .add_startup_system(load_notes_from_file)
        .add_system(input_detection.before(update_notes))
        .add_system(update_notes)
        .add_system(print_notes)
        .run();
}
