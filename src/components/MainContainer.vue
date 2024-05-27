<template>
    <div class="container">
        <n-space vertical>
            <ErrorAlerts />
            <ChangeDir :getPath="getPath" />
            <n-dialog-provider>
                <IDESelect />
            </n-dialog-provider>
        </n-space>
    </div>
</template>

<script lang="ts" setup>
import ErrorAlerts from "@/components/ErrorAlerts.vue";
import IDESelect from "@/components/IDESelect.vue";
import ChangeDir from "@/components/ChangeDir.vue";
import { useConfigStore } from "@/stores/config";
import { fromRegGetPath } from "@/rust";
import { useNotification } from 'naive-ui'

const store = useConfigStore();
const notification = useNotification()

getPath();

function getPath() {
    fromRegGetPath()
        .then((path: string) => {
            store.setToolboxDir(path);
        })
        .catch(() => {
            notification.warning({
                content: '从注册表读取toolbox路径失败，请手动指定',
                duration: 10000,
                keepAliveOnHover: true
            })
            store.setToolboxDir("");
        });
}
</script>

<style scoped>
.container {
    padding: 0 5vw;
    padding-top: 5vh;
    display: flex;
    flex-direction: column;
    text-align: center;
    min-height: 85%;
}
</style>