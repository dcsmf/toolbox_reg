<template>
  <n-config-provider :locale="zhCN">
  <n-notification-provider>
    <MainContainer />
  </n-notification-provider>
</n-config-provider>
</template>

<script lang="ts" setup>
import { zhCN } from 'naive-ui';
import MainContainer from './components/MainContainer.vue';
</script>