<template>
  <md-editor
    editorId="tauri-editor"
    :tabWidth="2"
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
      <normal-toolbar title="hi" @on-click="handler">
        <template #trigger>
          <!-- <svg class="md-icon" aria-hidden="true">
            <use xlink:href="#icon-fangda"></use>
          </svg> -->
            <!-- <svg t="1660660116448" class="md-icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="2293" width="200" height="200"><path d="M256 128a42.666667 42.666667 0 0 0 0 85.333333h512a42.666667 42.666667 0 0 0 0-85.333333H256z m256 205.994667L291.328 554.666667H426.666667v128a42.666667 42.666667 0 1 1-85.333334 0v-42.666667H291.328c-75.989333 0-114.090667-91.904-60.330667-145.664l220.672-220.672a85.333333 85.333333 0 0 1 120.661334 0l220.672 220.672c53.76 53.76 15.658667 145.664-60.330667 145.664H682.666667v213.333333a85.333333 85.333333 0 0 1-85.333334 85.333334h-170.666666a85.333333 85.333333 0 0 1-85.333334-85.333334 42.666667 42.666667 0 1 1 85.333334 0h170.666666v-298.666666H732.672L512 333.994667z" p-id="2294" fill="#ffffff"></path></svg> -->
            <svg t="1660660214547" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="3483" width="14" height="14"><path d="M921.395302 76.761619H102.604698c-20.981509 0-38.38081-17.3993-38.38081-38.380809S81.623188 0 102.604698 0h818.790604c20.981509 0 38.38081 17.3993 38.38081 38.38081s-17.3993 38.38081-38.38081 38.380809zM673.1994 1024h-317.281359c-16.375812 0-31.728136-6.14093-43.498251-17.911044s-17.911044-27.122439-17.911044-43.498251v-301.929036H139.962019c-16.375812 0-31.728136-6.14093-43.498251-17.911044s-17.911044-27.122439-17.911044-43.498251c0-15.352324 5.629185-30.192904 16.375812-41.451275l372.037981-405.813093c11.258371-12.281859 26.098951-18.934533 42.474762-19.958021 16.375812-0.511744 32.23988 5.117441 44.009995 15.864068l2.558721 2.558721 373.573213 406.836581c11.258371 12.281859 16.887556 27.634183 15.864068 44.009995-0.511744 16.375812-7.676162 31.728136-19.958021 42.474763-11.258371 10.234883-26.098951 16.375812-41.451274 16.375812h-149.429285v301.929036c0 16.375812-6.14093 31.728136-17.911045 43.49825-11.770115 12.281859-27.122439 18.422789-43.498251 18.422789z m-301.929035-76.761619h286.576711v-363.338331h191.392304L512 216.467766 174.76062 583.90005h196.509745v363.338331z m141.241379-738.446777z" p-id="3484" fill="#ffffff"></path></svg>
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
import { invoke } from "@tauri-apps/api/tauri";
import { BaseDirectory, writeTextFile, readTextFile } from "@tauri-apps/api/fs";
import { appDir, dataDir } from "@tauri-apps/api/path";
import { toolbar_titles } from "./toolbar";

const NormalToolbar = MdEditor.NormalToolbar;

const value_true = ref(true);
const value_false = ref(false);
const theme = ref("dark");

const toolbars = reactive(toolbar_titles);
// const toolbars = ['bold', '-', 0, '=', 'github'];

// 'default' | 'github' | 'vuepress' | 'mk-cute' | 'smart-blue' | 'cyanosis'
const previewTheme = ref("smart-blue");

const config = reactive({
  ding: false,
  title: "ding",
});

const handler = () => {
  console.log("NormalToolbar clicked!");
  console.log(appWindow);
  config.ding = !config.ding;
  appWindow.setAlwaysOnTop(config.ding);
  appWindow.setTitle(config.ding ? config.title + " - top" : config.title);
};

const syncHtml = async (content) => {
  // console.log(content)
  invoke("sync_html", { content: content });
  await writeTextFile("ding.html", content, { dir: BaseDirectory.Data });
};

const syncMarkdown = async (content) => {
  invoke("sync_md", { content: content });
  await writeTextFile("ding.md", content, { dir: BaseDirectory.Data });
};

const getMarkdown = async () => {
  try {
    const content = await readTextFile("ding.md", { dir: BaseDirectory.Data });
    return content;
  } catch (e) {
    console.log(e);
    return "";
  }
};

let html = await getMarkdown();
console.log(html);
const text = ref(html);

watch(text, (newValue, oldValue) => {
  console.log(newValue, oldValue);
  syncMarkdown(newValue);
});
</script>

<style scoped>
.icon {
    width: 14px;
    height: 14px;
    fill: currentColor;
    overflow: hidden;
}
</style>
