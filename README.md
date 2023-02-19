# rpush✨

## 介绍

一个推送本地文件到服务器空间的小工具。

工具可以保存多个服务器空间配置信息（主机地址、目标路径、用户名、密码），配置文件保存在当前用户目录，文件名：`.rpush_config` 。

## 用法

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

5. 将当前目录下的指定目录推送到指定服务器。这里要注意，<pushed_dir> 指的是当前目录下要推送的目录，推送到空间中的是该目录中的所有内容。
```bash
rpush push <pushed_dir> <space_name>
```

6. 删除服务器空间中的所有文件（使用的 rm -rf 命令）
```bash
rpush rmrf <space_name> 
```