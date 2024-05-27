import { invoke } from '@tauri-apps/api/tauri'
import type { ToolsDTO } from './types'

export async function checkToolboxExist(toolboxPath: string): Promise<boolean> {
  return await invoke('check_toolbox_exist', { toolboxPath })
}

export async function fromRegGetPath(): Promise<string> {
  return await invoke('from_reg_get_path')
}

export async function getStateFileStr(toolboxPath: string, fileName: string): Promise<string> {
  return await invoke('get_state_file_str', { toolboxPath, fileName })
}

export async function setIDEInfoToReg(tools: ToolsDTO[], toolboxPath: string):Promise<boolean> {
  return await invoke('set_ide_info_to_reg', { tools, toolboxPath })
}

export async function clearIDEInfoFromReg():Promise<boolean> {
  return await invoke('clear_ide_info_from_reg')
}