# Docs

## File loading

### read_file_to_string() returns String
if the number of cmd line arguments is 0 or 1 it will set _path to "no path" this is used by a later match statement for default paths,
if the number of cmd line arguments is greater than 1 it will take the first argument and make it the path.
```
let mut _path = match std::env::args().len() {
    0 => "no path".to_string(),
    1 => "no path".to_string(),
    _ => std::env::args().nth(1).unwrap(),
};
```

if _path is eqaul to "no path" it will be set to the default path which is "TaikoRS/Songs/.Debug/TestSong/", if it isnt it will be whatever the user inputed.
```
match _path.as_str() {
    "no path" => {_path = default_path.to_string() + "/Map.txt"},
    _ => {_path = _path + "/Map.txt"}
}
```

this checks if the path entered is valid if it isnt it will print an error message. later this will be written to a log file instead of the console
if it is valid it will read its contents to a string that as then returned ending the function
```
match std::fs::File::open(_path.to_string()) {
    Ok(_) => {
            let mut file = std::fs::File::open(_path).unwrap();
            file.read_to_string(&mut string).unwrap();
    },
    Err(e) => {println!("error opening file: error {}", e); string = "error".to_string();}
}
``` 

