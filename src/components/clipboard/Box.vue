<template>
    <div :class="{
        'box checked': info.checked,
        'box': !info.checked
    }">
        <input class="invisible" v-bind:autofocus="info.checked"/>
        <div class="header">{{ typeValue(info.content_type) }}</div>
        <div class="content">
            <div class="word">{{ info.content }}</div>
        </div>
        <div class="footer">{{ state.dateShow }}</div>
    </div>
</template>
<!-- v-bind:checked="info.checked" -->

<script setup>
import { defineProps, reactive, onMounted, onBeforeUnmount } from 'vue';
import { formatRelativeTime } from '../../util/util'

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
})

const typeValue = content_type => {
    if ("Text" === content_type) return "文本"
    return "其他类型"
}

onMounted(() => {
    state.intervalId = setInterval(() => {
        state.dateShow = formatRelativeTime(info.date * 1000)
    }, 1000)
})

onBeforeUnmount(() => {
    clearInterval(state.intervalId)
})

console.log("info:", info)

</script>

<style scoped>
.box {
    /* height: 100%; */
    margin: 14px;
    /* display: flex; */
    /* line-height: 30px; */
    align-items: center;
    /* justify-content: space-between; */
    /* flex-direction: column; */
    cursor: pointer;
    display: flex;
    flex-direction: column; /* 设置主轴为垂直方向 */
    /* height: 100vh; 为了演示效果，假设容器高度为视口高度 */
    justify-content: space-between; /* 头尾部分沿主轴均匀分布 */
}

.checked {
    outline: 1px solid red;
}

.invisible {
    /* visibility: hidden; */
    display: none;
}

.content {
    padding: 6px;
    width: 228px; /* 计算宽高比 */
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

.header {
    height: 26px;
    width: 100%;
    text-align: left;
    font-size: 16px;
    color: gray;
    padding-left: 8px;
}


.footer {
    height: 30px;
}


</style>