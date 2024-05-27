import { defineStore } from 'pinia'
import { type Config, type Tools } from '@/types'
import { computed, reactive } from 'vue'

export const useConfigStore = defineStore('config', () => {
  const globalConfig = reactive<Config>({
    toolboxDir: '',
    toolsArr: [],
    disable: true
  })
  const getConfig = computed(() => {
    return globalConfig
  })

  const getToolboxDir = computed(() => globalConfig.toolboxDir)

  const getToolsArr = computed(() => globalConfig.toolsArr)

  function setToolboxDir(path: string) {
    globalConfig.toolboxDir = path
  }

  function setToolsArr(arr: Tools[]) {
    globalConfig.toolsArr = arr
  }

  function setDisable(state: boolean) {
    globalConfig.disable = state
  }

  return {
    globalConfig,
    getConfig,
    getToolboxDir,
    getToolsArr,
    setToolboxDir,
    setToolsArr,
    setDisable
  }
})
