# 解决 Cursor 免费账户次数限制问题

白嫖 Cursor 免费账户次数过多之后会报如下错误：

![](./too_many.png)

可通过修改配置文件中的下面三个参数绕过：
```json
{
  "telemetry.macMachineId": "64位十六进制",
  "telemetry.machineId": "64位十六进制",
  "telemetry.devDeviceId": "UUID格式"
}
```
本项目基于上述想法编写，代码使用 rust 语言，直接 clone 项目并`cargo run`即可。

> 代码目前只在 MacOS 上经过验证有效，其他系统请自行测试。