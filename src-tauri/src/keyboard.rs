
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

pub fn paste() {

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let ctrl_press: Result<(), enigo::InputError> = enigo.key(Key::Control, Press);
    let v_click = enigo.key(Key::Unicode('v'), Click);
    let ctrl_release = enigo.key(Key::Control, Release);
    println!("{:?}", ctrl_press);
    println!("{:?}", v_click);
    println!("{:?}", ctrl_release);

}