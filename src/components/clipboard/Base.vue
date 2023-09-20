<template>
    <div class="main">
        <div>剪贴板</div>

    </div>
</template>

<script setup>
import { ref, reactive, watch, toRef, toRefs } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import { appWindow, primaryMonitor, WebviewWindow } from "@tauri-apps/api/window";
import { union_list } from "../../util/util";

const webview = new WebviewWindow('clipboard');
listen('CLIPBOARD_UPDATE', async (event) => {
    console.log('clipboard update: ', event)
    let data = await clips(state.last_id)
    console.log('data: ', data)
    let old_list = toRefs(state.list)
    console.log('old_list: ', old_list)
    let new_list = union_list(data, old_list)
    state.list = new_list
    console.log('list: ', new_list);
})

const state = reactive({
    last_id: 0,
    list: []
})

const clips = async (last_id) => {
    let data = invoke("get_clipboard", { lastId: last_id });
    console.debug('receive data: ', data)
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