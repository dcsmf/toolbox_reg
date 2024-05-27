<template>
  <n-transfer ref="transfer" v-model:value="valueStr" :options="options" :disabled="disable" />
  <n-space vertical>
    <n-button @click="getIDEList" :disabled="disable">读取工具列表</n-button>
    <n-button :disabled="disable" @click="setIDEInfo" type="info">写入文件夹右键菜单</n-button>
    <n-popconfirm @positive-click="clearIDEReg">
      <template #trigger>
        <n-button type="warning">清除文件夹右键菜单</n-button>
      </template>
      确认要清理本软件生成的注册表？
    </n-popconfirm>
  </n-space>

</template>
<script setup lang="ts">
import { computed, ref } from 'vue'
import { useConfigStore } from '@/stores/config'
import { clearIDEInfoFromReg, getStateFileStr, setIDEInfoToReg } from '@/rust'
import type { Tools, ToolsDTO } from '@/types'
import { useDialog } from 'naive-ui'

const toolsList = ref<Tools[]>([])
const valueStr = ref<string[]>([])
const store = useConfigStore()
const options = ref(createOptions())
const dialog = useDialog()

const disable = computed(() => {
  return store.getConfig.disable
})

function createOptions() {
  return toolsList.value.map((t) => ({
    label: t.displayName + " " + t.displayVersion,
    value: JSON.stringify(t)
  }))
}

async function setIDEInfo() {
  let toolsArray = valueStr.value;
  if (!toolsArray || toolsArray.length === 0) {
    dialog.error({
      title: '错误',
      content: '请选择要写入的工具',
      positiveText: '确认',
      closable: false
    })
    return;
  }
  let tools = getToolsInfo(toolsArray)
  setIDEInfoToReg(tools, store.getToolboxDir)
    .then(() => {
      popDialog('success', '已经写入注册表')
    })
    .catch((err) => {
      popDialog('error', '写入注册表失败，请检查是否有管理员权限，错误：' + err)
    })
}

function popDialog(type: "default" | "error" | "info" | "success" | "warning", content: string) {
  dialog.create({
    type: type,
    title: '提示',
    content: content,
    positiveText: '确认',
    closable: false
  })
}

function getToolsInfo(tools: string[]): ToolsDTO[] {
  return tools
    .map(c => JSON.parse(c))
    .map(c => (
      {
        tag: c.tag,
        name: c.displayName,
        location: c.installLocation + '\\' + c.launchCommand.replace('/', '\\')
      } as ToolsDTO
    ));
}

function getIDEList() {
  getStateFileStr(store.getToolboxDir, "state.json").then((res) => {
    if (!res || res === '') {
      return;
    }
    let json: { tools: Tools[] } = JSON.parse(res)
    store.setToolsArr(json.tools)
    toolsList.value = json.tools
    options.value = createOptions()
  })
  popDialog('info', '读取成功')
}

function clearIDEReg() {
  clearIDEInfoFromReg()
    .then(() => {
      popDialog('success', "清理成功！")
    })
    .catch((err) => {
      popDialog('error', "清理失败：" + err)
    })
}




</script>
