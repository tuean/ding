<template>
  <md-editor
    editorId="tauri-editor"
    :tabWidth=2
    :showCodeRowNumber="value_true"
    v-model="text"
    :theme="theme"
    :pageFullScreen="value_true"
    :preview="value_false"
    :htmlPreview="value_false"
    :noPrettier="value_true"
    :previewTheme="previewTheme"
    :toolbars="toolbars"
    :onHtmlChanged="syncHtml"
  >
    <template #defToolbars>
      <normal-toolbar title="mark" @on-click="handler">
        <template #trigger>
          <svg class="md-icon" aria-hidden="true">
            <use xlink:href="#icon-mark"></use>
          </svg>
        </template>
      </normal-toolbar>
    </template>
  </md-editor>
</template>

<script setup>
import { ref, reactive, watch } from "vue";
import MdEditor from "md-editor-v3";
import "md-editor-v3/lib/style.css";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { invoke } from '@tauri-apps/api/tauri'
import { BaseDirectory, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
import { appDir, dataDir } from '@tauri-apps/api/path';
import { toolbar_titles } from './toolbar'

const NormalToolbar = MdEditor.NormalToolbar;

const value_true = ref(true)
const value_false = ref(false)
const theme = ref("dark");

const toolbars = reactive(toolbar_titles);
// const toolbars = ['bold', '-', 0, '=', 'github'];

// 'default' | 'github' | 'vuepress' | 'mk-cute' | 'smart-blue' | 'cyanosis'
const previewTheme = ref("smart-blue");

const config = reactive({
    ding: false,
    title: "ding"
})

const handler = () => {
  console.log("NormalToolbar clicked!");
  console.log(appWindow)
  config.ding = !config.ding
  appWindow.setAlwaysOnTop(config.ding);
  appWindow.setTitle(config.ding ? config.title + " - top" : config.title);
};

const syncHtml = async content => {
    // console.log(content)
    invoke('sync_html', { "content": content })
    await writeTextFile('ding.html', content, { dir: BaseDirectory.Data });
}

const syncMarkdown = async content => {
    invoke('sync_md', { "content": content })
    await writeTextFile('ding.md', content, { dir: BaseDirectory.Data });
}

const getMarkdown = async () => {
    try {
        const content = await readTextFile('ding.md', { dir: BaseDirectory.Data });
        return content
    } catch (e) {
        console.log(e)
        return '';
    }
}

let html = await getMarkdown()
console.log(html)
const text = ref(html)

watch(text, (newValue, oldValue) => {
    console.log(newValue, oldValue)
    syncMarkdown(newValue)
})

</script>
