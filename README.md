# rpush✨

---

正在学习 Rust，做了一个小工具。

将当前目录下的指定目录推送到服务器。

工具可以添加保存 N 个服务器空间配置（主机地址、目标路径、用户名等信息）。

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

---

服务器配置默认保存在当前系统用户目录，`.rpush_config` 。