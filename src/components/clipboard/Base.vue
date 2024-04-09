<template>
    <div class="main">
        <!-- <div>剪贴板</div> -->
        <div class="content-box" ref="container">
            <Box v-for="(info, index) in state.list" :info="info" :index="index"/>
        </div>

    </div>
</template>

<script setup>
import { ref, reactive, watch, toRef, toRefs, unref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import { appWindow, primaryMonitor, WebviewWindow } from "@tauri-apps/api/window";
import { union_list, set_checked } from "../../util/util";
import Box from './Box.vue'

const state = reactive({
    last_id: 0,
    list: []
})

const clips = async (last_id) => {
    let data = invoke("get_clipboard", { lastId: last_id });
    console.debug('receive data: ', data)
    return data;
}



// 滚轮事件监听
const container = ref(null);
const handleWheel = (event) => {
    // 阻止默认的垂直滚动行为  
    event.preventDefault();

    // 计算滚动的距离和方向  
    const deltaX = event.deltaY * 1; // 使用deltaY表示滚动的距离，乘以-1表示反方向  
    const left = container.value.scrollLeft;
    console.log("left", left)
    const scrollLeft = left + deltaX;

    // 设置滚动距离  
    container.value.scrollLeft = scrollLeft;
};

onMounted(() => {
    // 添加滚动事件监听器  
    container.value.addEventListener('wheel', handleWheel, { passive: false });


});

const init = () => {
        // 剪贴板事件监听
        const webview = new WebviewWindow('clipboard');
    debugger
    listen("CLIPBOARD_UPDATE", async (event) => {
        console.log('clipboard update: ', event)
        let data = await clips(state.last_id)
        console.log('data: ', data)
        let old_list = state.list
        console.log('old_list: ', old_list)
        let new_list = union_list(data, old_list)
        set_checked(new_list); // 给数据添加checked参数
        state.list = new_list 
        console.log('list: ', new_list);
    })
}

init();


</script>

<style scoped>
.main {
    width: 100vw;
    /* height: 300px; */
    min-height: 100px;
    height: 90vh;
    position: static;
    display: flex;
}

.content-box {
    height: 100%;
    display: flex;
    overflow-x: auto;
    overflow-y: hidden;
    scroll-behavior: smooth;
    scrollbar-color: gray;
    scrollbar-width: thin;
}
</style>