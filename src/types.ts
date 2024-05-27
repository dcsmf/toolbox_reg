export type Config = {
  toolboxDir: string
  toolsArr: Tools[]
  disable: boolean
}

export type Tools = {
  /**
   * "channelId": "9a6b3c2f-3b10-4e4d-a132-39df61f84cb8",
   * "toolId": "IDEA-U",
   * "productCode": "IU",
   * "tag": "idea",
   * "displayName": "IntelliJ IDEA Ultimate",
   * "displayVersion": "2023.3.2",
   * "buildNumber": "233.13135.103",
   * "installLocation": "D:\\IntelliJ IDEA Ultimate",
   * "launchCommand": "bin/idea64.exe"
   */
  toolId: string
  tag: string
  displayName: string
  displayVersion: string
  installLocation: string
  launchCommand: string
}

export type ToolsDTO = {
  tag: string
  name: string
  location: string
}
