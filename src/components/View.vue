<template>
    <div v-html="text" class="showing" @dblclick="createTopWindow" data-tauri-drag-region>

    </div>
</template>

<script setup>
import { BaseDirectory, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
import { ref, triggerRef } from 'vue';
import { WebviewWindow } from '@tauri-apps/api/window'


const getHtml = async () => {
    try {
        const content = await readTextFile('ding.html', { dir: BaseDirectory.Data });
        console.log('content', content)
        return content
    } catch (e) {
        console.log(e)
        return '<div></div>'
    }
}

const html = await getHtml()
console.log(html)
const text = ref(html)


const createTopWindow = () => {
    console.log("double click, start to create a new top window")
    let label = Math.random().toString(36).slice(-8);
    const webview = new WebviewWindow(label, {
        url: '/#/viewTop',
        resizable: true,
        title: "ding-" + label,
        width: 300,
        height: 100,
        focus: true,
        transparent: true,
        decorations: false,
        alwaysOnTop: true,
        skipTaskbar: false,
    })
    // since the webview window is created asynchronously,
    // Tauri emits the `tauri://created` and `tauri://error` to notify you of the creation response
    webview.once('tauri://created', function () {
    // webview window successfully created
    })
    webview.once('tauri://error', function (e) {
    // an error occurred during webview window creation
    })
}


</script>

<style scoped>
.showing {
    overflow-x: hidden;
    user-select: none;
    /* overflow-y: scroll; */
    /* width: 100%; */
}
</style>