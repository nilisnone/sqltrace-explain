# 环境配置

如果配置了 SOI_HOME ，则将之设置为工作环境目录，否则当前运行的目录为工作目录

工作目录中需要配置 config/app.yaml 和 config/{env}.yaml 两个配置文件，其中 env 需要通过

SOI_ENV 配置，或者默认为 local.yaml，这么设置的主要目的是：

- app.yaml 存储所有的配置，有完整的说明和默认值
- {env}.yaml 可以覆盖任何配置，用于自定义


# todo

- [x] config 配置
- [ ] 读取并监控 scan_dir 目录下的日志文件