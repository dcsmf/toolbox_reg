<template>
  <div class="root-change-dir" style="display: flex;">
    <n-row>
      <n-col :span="18">
        <n-input v-model:value="dir" type="text" placeholder="请选择jetbrains-toolbox.exe所在的目录" style="flex: 1;" />
      </n-col>
      <n-col :span="3">
        <n-button ref="chooseDirButton" type="primary" @click="chooseDir">选择目录</n-button>
      </n-col>
      <n-col :span="3">
        <n-button @click="resetDir">从注册表读取</n-button>
      </n-col>
    </n-row>
  </div>
</template>

<script setup lang="ts">
import { useConfigStore } from "@/stores/config";
import { computed } from "vue";
import { open } from '@tauri-apps/api/dialog';
import { appDataDir } from '@tauri-apps/api/path';

const props = defineProps<{
  getPath: Function;
}>();

const store = useConfigStore();

const dir = computed({
  get() {
    return store.getToolboxDir;
  },
  set(val) {
    store.setToolboxDir(val);
  },
});

async function chooseDir() {
  let path;
  if (store.getToolboxDir) {
    path = store.getToolboxDir
  } else {
    path = await appDataDir()
  }
  let selected = await open({
    directory: true,
    multiple: false,
    defaultPath: path,
  });
  if (Array.isArray(selected)) {
    // 用户选择了多个目录
  } else if (selected === null) {
    // 用户取消了选择
  } else {
    // 用户选择了一个目录
    store.setToolboxDir(selected)
  }
}

function resetDir() {
  props.getPath();
}
</script>