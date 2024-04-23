<template>
    <div class="box" v-bind:checked="info.checked">
        <div class="content">{{ info.content }}</div>
    </div>
</template>

<script setup>
import { defineProps } from 'vue';

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

console.log("info:", info)

</script>

<style scoped>
.box {
    height: 100%;
    padding: 8px;
    display: flex;
    /* line-height: 30px; */
    align-items: center;
    justify-content: center;
    cursor: pointer;
}

.box:checked {
    outline: 1px solid red;
    background-color: #f2f2f2; /* 选中状态的背景色 */  
    border-color: #007bff; /* 选中状态的边框色 */  
}

.content {
    padding: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 20vw;
    min-width: 10vw;
    width: 10vw
}


</style>