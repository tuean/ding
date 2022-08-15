import { isRegistered, register } from '@tauri-apps/api/globalShortcut';


export const registerShortcut = async (key, fn) => {
    const registed = await isRegistered(key);
    if (!registed) {
        // alert(registed)
        await register(key, fn);
    } else {
        console.log('key is registed', key)
    }
}