<template>
  <div data-tauri-drag-region class="drag" v-if="state.dragable">
    <svg @click="quit_window" t="1692371605182" class="icon minus" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="31312" xmlns:xlink="http://www.w3.org/1999/xlink" width="200" height="200"><path d="M191.977138 511.981711c0 176.793684 143.194885 319.988569 319.988569 319.988569 57.797935 0 111.995999-15.39945 158.794327-42.198493l-143.994856-172.793827c-22.599193-27.199028-18.999321-67.597585 8.199707-90.196778s67.597585-18.999321 90.196778 8.199707l141.794935 170.19392c40.798543-53.598085 64.997678-120.595692 64.997678-193.193098 0-176.793684-143.194885-319.988569-319.988569-319.98857S191.977138 335.188026 191.977138 511.981711z m561.779931 377.186526C683.959563 933.966636 600.962528 959.965707 511.965707 959.965707 264.574544 959.965707 63.98171 759.372873 63.98171 511.981711S264.574544 63.997714 511.965707 63.997714s447.983997 200.592834 447.983997 447.983997c0 112.195992-41.198528 214.792327-109.396092 293.389519l94.59662 113.595942c22.599193 27.199028 18.999321 67.597585-8.199707 90.196778s-67.597585 18.999321-90.196778-8.199707l-93.19667-111.796006z" p-id="31313"></path></svg>
    <svg @click="minus_window" t="1692367700631" class="icon minus" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="31171" xmlns:xlink="http://www.w3.org/1999/xlink" width="200" height="200"><path d="M928 512c0 35.4-28.6 64-64 64H160c-35.4 0-64-28.6-64-64s28.6-64 64-64h704c35.4 0 64 28.6 64 64z" p-id="31172"></path></svg>
  </div>
  <Suspense >
      <router-view class="content" />
  </Suspense>
  
</template>

<script setup>
import Editor from "./components/Editor.vue";
import { registerShortcut } from "./util/shortcut";
import { reactive, onMounted } from "vue";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
import { useRouter } from "vue-router";
import { appWindow, primaryMonitor, WebviewWindow } from "@tauri-apps/api/window";




const monitor = primaryMonitor();

console.log('monitor', monitor)

console.log("register start");
const state = reactive({
  mode: true,
  dragable: true
});
const modelChange = "CommandOrControl+U"; 
const exit = "exit"
const router = useRouter();

const unregiste = async (key) => {
  await unregister(key);
};

const registe = async (key) => {
  await register(key, () => {
    console.log(state.mode);
    state.mode = !state.mode;
    router.push({
      path: state.mode ? "/app/editor" : "/app/view",
    });
  });
};

const registed = isRegistered(modelChange);
if (registed) {
  // const un = unregiste(modelChange);
  console.log("快捷键 ctrl + U 已被注册")
}
const result = registe(modelChange);

console.log('window', appWindow)

appWindow.listen("LogicalPosition", ({ event, payload }) => {
  console.log('listen', event, payload)
 });

 const minus_window = async () => {
  console.log('minus window');
  appWindow.minimize();
 }

 const quit_window = async () => {
  console.log('quit window');
  appWindow.close();
 }


onMounted(() => {
  let local_path = window.location.hash
  console.log('local path', local_path)
  if (local_path === '#/clipboard') {
    state.dragable = false
  } else {
    state.dragable = true
  }
})
</script>

<style scoped>
.drag {
  height:10vh;
  width: 100%;
  /* background-color: aqua; */
  top: 0;
  z-index: 9999;
  position: fixed;
  display: flex;
  flex-direction: row-reverse;
}
.content {
  /* margin-top: 24px; */
  padding-top: 1px;
}

.minus {
  z-index: 999999;
    color: white;
    height: 24px;
    width: 36px;
}
.minux:hover {
  cursor: pointer;
}
</style>
