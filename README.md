# rpush✨

正在学习 Rust，做了一个小工具。

功能是将当前目录下的指定目录推送到服务器。

工具可以保存多个服务器空间配置信息（主机地址、目标路径、用户名等），配置文件保存在当前用户目录下，文件名：`.rpush_config` 。

## Usage

1. 添加服务器配置
```bash
rpush add 
```

2. 列出已添加的服务器配置
```bash
rpush list 
```

3. 查看服务器配置详情
```bash
rpush detail <space_name>
```

4. 移除服务器配置
```bash
rpush remove <space_name>
```

5. 将当前目录下的指定目录推送到指定服务器
```bash
rpush push <pushed_dir> <space_name>
```

6. 删除服务器空间中的所有文件（使用的 rm -rf 命令）
```bash
rpush rmrf <space_name> 
```