<template>
    <div data-tauri-drag-region v-html="text" class="showing" >

    </div>
</template>

<script setup>
import { BaseDirectory, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
import { ref } from 'vue';
import { WebviewWindow } from '@tauri-apps/api/window'


const getHtml = async () => {
    try {
        const content = await readTextFile('ding.html', { dir: BaseDirectory.Data });
        console.log('view top content', content)
        return content
    } catch (e) {
        console.log(e)
        return '<div></div>'
    }
}

const html = await getHtml()
console.log(html)
const text = ref(html)


</script>

<style scoped>
.showing {
    overflow-x: hidden;
    user-select: none;
}
</style>