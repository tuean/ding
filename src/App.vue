<template>
  <!-- <Suspense>
    <Editor v-if="state.mode"/>
  </Suspense>
  <Suspense>
    <View v-if="!state.mode" />
  </Suspense> -->
  <!-- <div>11</div> -->
  <Suspense>
      <router-view />
  </Suspense>
  
</template>

<script setup>
import Editor from "./components/Editor.vue";
import { registerShortcut } from "./util/shortcut";
import { reactive } from "vue";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
import { useRouter } from "vue-router";
import { appWindow, primaryMonitor } from "@tauri-apps/api/window";
import {  } from '@tauri-apps/api/window';
const monitor = primaryMonitor();

console.log('monitor', monitor)

console.log("register start");
const state = reactive({
  mode: true,
});
const modelChange = "CommandOrControl+U";
const router = useRouter();

const unregiste = async (key) => {
  await unregister(key);
};

const registe = async (key) => {
  await register(key, () => {
    console.log(state.mode);
    state.mode = !state.mode;
    router.push({
      // name: state.mode ? 'editor' : 'view'
      path: state.mode ? "/app/editor" : "/app/view",
    });
  });
};

const registed = isRegistered(modelChange);
if (registed) {
  const un = unregiste(modelChange);
}
const result = registe(modelChange);

console.log('window', appWindow)

appWindow.listen("LogicalPosition", ({ event, payload }) => {
  console.log('listen', event, payload)
 });

</script>

<style scoped></style>
