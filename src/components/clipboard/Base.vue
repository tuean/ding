<template>
    <div class="main">
        <div>剪贴板</div>

    </div>
</template>

<script setup>
import { ref, reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from '@tauri-apps/api/event'
import { appWindow, primaryMonitor, WebviewWindow } from "@tauri-apps/api/window";

const webview = new WebviewWindow('clipboard');
listen('CLIPBOARD_UPDATE', async (event) => {
    console.log('clipboard update: ', event)
    let data = await clips(state.last_id)
})

const state = reactive({
    last_id: 0,
})

const clips = async (last_id) => {
    let data = invoke("get_clipboard", { lastId: last_id });
    console.log('receive data: ', data)
    return data;
}

</script>

<style scoped>
.main {
    width: 100vw;
    height: 300px;
    position:static;
    display: flex;
}
</style>