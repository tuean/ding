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
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
