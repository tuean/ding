<template>
  <md-editor :toolbars="toolbars">
    <template #defToolbars>
      <normal-toolbar title="mark" @on-click="handler">
        <template #trigger>
          <svg class="md-icon" aria-hidden="true">
            <use xlink:href="#icon-mark"></use>
          </svg>
        </template>
      </normal-toolbar>
    </template>
  </md-editor>
</template>

<script setup>
import MdEditor from 'md-editor-v3';
const NormalToolbar = MdEditor.NormalToolbar;

const toolbars = ['bold', '-', 0, '=', 'github'];

const handler = () => {
  console.log('NormalToolbar clicked!');
};
</script>
