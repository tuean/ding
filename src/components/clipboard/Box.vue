<template>
    <div :class="{
        'box checked': info.checked,
        'box': !info.checked
    }">
        <input class="invisible" />
        <div class="header">
            <div class="left">{{ typeValue(info.content_type) }}</div>
            <div class="right"><span v-show="'Text' == info.content_type">{{ size_length(info) }}个字符</span></div>
        </div>
        <div class="content">
            <!-- <div>content</div> -->
            <div v-if="info.content_type === 'Text'" class="word"><div>{{ info.content }}</div></div>
            <div v-if="info.content_type === 'Unknown'" class="html noclick"><div class="html-wrapper" v-html="info.content"></div></div>
            <div v-if="info.content_type === 'Html'" class="html noclick"> <div v-html="info.content"></div></div>
            <div v-if="info.content_type === 'Rtf'" class="word"><div>{{ info.content }}</div></div>
            <div v-if="info.content_type === 'File'" class="word">
                <div v-for="item in info.content.split(';')">
                    <div>{{ item.split('\\').pop() }}</div>
                </div>
            </div>
            <div v-if="info.content_type === 'Image'" class="image">
                <img :src="state.image_data" class="img" />
            </div>
        </div>
        <div class="footer">{{ state.dateShow }}</div>
    </div>
</template>
<!-- v-bind:checked="info.checked" -->
<!-- v-myfocus="info.checked" -->
<script setup>
import { defineProps, reactive, onMounted, onBeforeUnmount } from 'vue';
import { formatRelativeTime } from '../../util/util'
import { readBinaryFile } from '@tauri-apps/api/fs';


const { info } = defineProps({
    info: {
        type: Object,
        default: {}
    },
    index: {
        type: Number,
        default: null
    }
})

const state = reactive({
    dateShow: "",
    intervalId: null,
    image_data: "",
})

const load_image = async () => {
    let path = info.content
    const binary_data = await readBinaryFile(path);
    let binary_data_arr = new Uint8Array(binary_data);
	let p = new Blob([binary_data], {type: 'image/png'});
	state.image_data = URL.createObjectURL(p);
}

const typeValue = content_type => {
    if ("Text" === content_type) return "文本"
    if ("Html" === content_type) return "网页"
    if ("File" === content_type) return "文件"
    if ("Image" === content_type) return "图片"
    if ("Rtf" === content_type) return "富文本"
    return "其他类型"
}

const size_length = info => {
    if ("Text" != info.content_type) return ""
    return info.content.length
}

onMounted(() => {
    state.intervalId = setInterval(() => {
        state.dateShow = formatRelativeTime(info.date * 1000)
    }, 1000)
    if (info.content_type === 'Image') {
        load_image();
    }
})

onBeforeUnmount(() => {
    clearInterval(state.intervalId)
})

console.log("info:", info)

</script>

<style scoped>
.box {
    user-select: none;
    margin: 14px;
    align-items: center;
    cursor: pointer;
    display: flex;
    flex-direction: column; 
    /* height: 100vh; 为了演示效果，假设容器高度为视口高度 */
    justify-content: space-between; 
}

.checked {
    outline: 4px solid purple;
    outline-offset: 2px;
    /* -moz-outline-radius: 8px; */
}

.invisible {
    /* visibility: hidden; */
    display: none;
}

.noclick {
    pointer-events: none;
}

.content {
    padding: 6px;
    min-width: 228px; /* 计算宽高比 */
    max-width: 600px;
    flex-grow: 1; /* 让内容区域占据剩余空间 */
    display: flex;
    flex-direction: column;
    justify-content: center; /* 内容区域垂直居中 */
}

.word {
    text-overflow: ellipsis;
    white-space: nowrap;
    overflow: hidden;
}

.image {
    overflow: hidden;
    display: flex;
    height: 50vh;
    justify-content: space-around;
}

.img {
    height: auto;
    width: auto;
    overflow: hidden;
    object-fit: contain;
}

.html {
    display: flex;
    flex-direction: column;
    align-content: center;
    justify-content: space-around;
    word-wrap: break-word;
    height: 50vh;
    flex-wrap: nowrap;
    overflow-x: auto;
    overflow: auto;
}

.html-wrapper {
    max-width: 200px;
    object-fit: cover;
    overflow: hidden;
}

.header {
    height: 26px;
    width: 100%;
    text-align: left;
    font-size: 16px;
    color: gray;
    /* padding-left: 8px; */
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}


.footer {
    height: 30px;
    color: gray;
}

.left {
    padding-left: 2px;
}
.right {
    padding-right: 2px;
}


</style>