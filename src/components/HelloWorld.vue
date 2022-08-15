<script setup>
import { ref } from "vue";

import { window } from "@tauri-apps/api";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification
} from "@tauri-apps/api/notification";

// 窗口置顶
const handleWindowTop = () => {
  let curWin = window.getCurrent();
  if (datas.winTop === "窗口置顶") {
    curWin.setAlwaysOnTop(true);
    datas.winTop = "取消置顶";
  } else {
    curWin.setAlwaysOnTop(false);
    datas.winTop = "窗口置顶";
  }
};

// appWindow.center();
appWindow.setAlwaysOnTop(true);
appWindow.setResizable(true);
appWindow.setSkipTaskbar(false);

// notification
// let permissionGranted = await isPermissionGranted();
// if (!permissionGranted) {
//   const permission = await requestPermission();
//   permissionGranted = permission === "granted";
// }
// if (permissionGranted) {
//   sendNotification({ title: "TAURI", body: "Tauri is awesome!" });
// }

defineProps({
  msg: String
});

const count = ref(0);
</script>

<template>
  <div>ss</div>
  <!-- <h1>{{ msg }}</h1> -->
  <!-- <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR2
    </p>
  </div>-->
  <!-- <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>-->
  <!-- <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>-->
  <!-- <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p> -->
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
