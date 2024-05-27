<template>
  <n-alert v-if="!toolboxExist" title="警告" type="error">
    <div v-html="text"></div>
  </n-alert>
</template>

<script setup lang="ts">
import { useConfigStore } from "@/stores/config";
import { ref } from "vue";
import { computedAsync } from "@vueuse/core";
import { checkToolboxExist } from "@/rust";

const store = useConfigStore();
const text = ref("Toolbox未找到，请确认目录是否正确！");
const toolboxExist = computedAsync(async () => {
  return checkToolboxExist(store.getToolboxDir)
    .then((val: boolean) => {
      store.setDisable(!val)
      return val;
    })
});
</script>
