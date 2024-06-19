<template>
    <div class="main">
        <!-- <div>剪贴板</div> -->
        <div class="content-box" ref="container">
            <Box :id="info.id" 
            @dblclick="copy(info, index)"
            @click="choose(info, index)" v-for="(info, index) in state.list" :key="info.id"
                :info="info" :index="index" />
        </div>
        <!-- :ref="(el) => refsCollection.set(info.id, el)"  -->
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
    current_index: 0,
    list: []
})

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

const clips = async (last_id) => {
    let data = invoke("get_clipboard", { lastId: last_id });
    return data;
}

const clips_older = async (id) => {
    let data = invoke("get_older", { lastId: id });
    return data;
}

const scrollTo = () => {
    if (state.list.length < 1) return;
    let el = document.getElementById(state.list[state.current_index].id);
    console.log("el", el)
    el.scrollIntoView({
        block: "start",
        behavior: "smooth",
        inline: "center"
    });
}

const history_check = () => {
    let max = state.list.length
    if (state.current_index > max - 5) {
        let id = state.list[state.current_index].id
        load_old_data(id)
    }
}

const next = () => {
    let max = state.list.length
    history_check();
    if (state.current_index >= max - 1) return;
    state.current_index++
    set_checked(state.list, state.current_index)
    scrollTo()
}

const last = () => {
    if (state.current_index == 0) return;
    state.current_index--
    set_checked(state.list, state.current_index)
    scrollTo()
}

const choose = (info, index) => {
    state.current_index = index
    history_check()
    set_checked(state.list, state.current_index)
    scrollTo()
}

const copy = async (info, index) => {
    choose(info, index)
    // invoke("do_copy", { id: info.id });
    console.log("appWindow hide", appWindow)
    appWindow.hide();
    invoke('do_paste', { id: info.id });
}

onMounted(() => {
    // 添加滚动事件监听器  
    container.value.addEventListener('wheel', handleWheel, { passive: false });

    // 键盘事件处理  
    document.addEventListener('keydown', function (event) {
        // console.log('key event', event)
        switch (event.key) {
            case 'ArrowUp':
                console.log('ArrowUp')
                last()
                break;
            case 'ArrowDown':
                console.log('ArrowDown')
                next()
                break;
            case 'ArrowLeft':
                console.log('ArrowLeft')
                last();
                break;
            case 'ArrowRight':
                console.log('ArrowRight')
                next();
                break;
            case 'Enter':
                console.log('enter')
                if (state.list.length < 1 || state.current_index < 1) break;
                copy(state.list[state.current_index], state.current_index);
                break;
            default:
                return;
        }

        // 将焦点设置到当前选中的项目，以便可以通过Tab键切换  
        // items[currentIndex].focus();  

        // 阻止默认行为和冒泡  
        event.preventDefault();
        event.stopPropagation();
    });
});



const load_newest_data = async (reset_index) => {
    let data = await clips(state.last_id)
    console.log('data: ', data)
    let old_list = state.list
    console.log('old_list: ', old_list)
    let new_list = union_list(data, old_list)
    if (reset_index) {
        set_checked(new_list, 0); // 给数据添加checked参数
        state.current_index = 0;
    }
    state.list = new_list
    console.log('list: ', new_list);
}

const load_old_data = async (id) => {
    let data = await clips_older(id)
    let old_list = state.list
    console.log('old_list: ', old_list)
    let new_list = union_list(data, old_list)
    // if (reset_index) {
        // set_checked(new_list, 0); // 给数据添加checked参数
        // state.current_index = 0;
    // }
    state.list = new_list
    console.log('list: ', new_list);
}

const init = () => {
    // 剪贴板事件监听
    const webview = new WebviewWindow('clipboard');
    // debugger
    listen("CLIPBOARD_UPDATE", async (event) => {
        console.log('clipboard update: ', event)
        load_newest_data(true)
        scrollTo()
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