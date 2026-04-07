# Pizza - Rust 重构实施计划
精简范围说明

**包含:** 核心网关、反向代理、静态文件服务、后端代理(HTTP/TCP/Transparent)、中间件链、流量控制、熔断器、gRPC 管理面、WASM 插件、统计、健康探测、延迟测量、ACME 证书管理

**排除:** VPN/Trojan/DNS/AnyTLS/WebDAV 实验性服务

## 完整项目结构

```toml
pizza/
├── Cargo.toml
├── build.rs
├── proto/
│   └── service.proto
├── config/
│   └── config.json          # 从父项目复制
└── src/
    ├── main.rs              # CLI 入口
    ├── lib.rs               # 模块声明
    ├── error.rs             # 统一错误
    ├── constants.rs         # 常量定义
    ├── config/              # 配置系统
    │   ├── mod.rs
    │   ├── loader.rs
    │   ├── merge.rs
    │   ├── app_config.rs
    │   ├── server_config.rs
    │   ├── middleware_config.rs
    │   ├── feature_config.rs
    │   ├── backend_config.rs
    │   └── frontend_config.rs
    ├── app/                 # 应用核心
    │   ├── mod.rs
    │   ├── application.rs
    │   ├── lifecycle.rs
    │   └── status.rs
    ├── gateway/             # 网关核心
    │   ├── mod.rs
    │   ├── proxy.rs
    │   ├── director.rs
    │   ├── transport.rs
    │   ├── manager.rs
    │   ├── resolver.rs
    │   ├── balancer.rs
    │   ├── proxy_cache.rs
    │   ├── autocert.rs
    │   └── server/
    │       ├── mod.rs
    │       ├── http_server.rs
    │       └── h3_server.rs
    ├── middleware/          # 中间件
    │   ├── mod.rs
    │   ├── pre_handler/
    │   │   ├── mod.rs
    │   │   ├── auth.rs
    │   │   ├── sanitizer.rs
    │   │   ├── domain_ctrl.rs
    │   │   ├── rate_limiter.rs
    │   │   └── image_protect.rs
    │   └── modifier/
    │       ├── mod.rs
    │       ├── trace_id.rs
    │       ├── secure_header.rs
    │       ├── cors.rs
    │       ├── gzip.rs
    │       ├── custom_header.rs
    │       └── fail_response.rs
    ├── flow_control/        # 流量控制
    │   ├── mod.rs
    │   ├── fixed_window.rs
    │   ├── leaky_bucket.rs
    │   ├── token_bucket.rs
    │   └── sliding_window.rs
    ├── breaker/             # 熔断器
    │   └── mod.rs
    ├── frontend/            # 前端静态代理
    │   ├── mod.rs
    │   ├── server.rs
    │   ├── static_handler.rs
    │   └── file_cache.rs
    ├── backend/             # 后端代理
    │   ├── mod.rs
    │   ├── proxy.rs
    │   ├── http_svr.rs
    │   └── tcp_proxy.rs
    ├── static_direct/       # 静态直连
    │   └── mod.rs
    ├── grpc/                # gRPC
    │   ├── mod.rs
    │   ├── server.rs
    │   ├── service.rs
    │   ├── proxy.rs
    │   └── web_proxy.rs
    ├── wasm_plugin/         # WASM 插件
    │   └── mod.rs
    ├── stat/                # 统计
    │   ├── mod.rs
    │   ├── collector.rs
    │   └── geo.rs
    ├── health_probe/        # 健康探测
    │   └── mod.rs
    ├── latency/             # 延迟测量
    │   └── mod.rs
    ├── notifier/            # 通知
    │   └── mod.rs
    ├── error_page/          # 错误页面
    │   └── mod.rs
    ├── initialize/          # 初始化系统
    │   ├── mod.rs
    │   └── registry.rs
    └── utils/               # 工具
        ├── mod.rs
        ├── time.rs
        ├── trace_id.rs
        ├── header.rs
        └── defaults.rs
```



## 分步实施清单

### Phase 1: 基础设施 (6 个文件)

1. Cargo.toml - 依赖配置

2. build.rs - protobuf 编译

3. src/constants.rs - 代理模式/协议/内部标志常量

4. src/error.rs - thiserror 定义所有错误类型

5. src/utils/ - 时间/trace_id/header/默认值工具

6. src/config/ - 配置加载器(JSON/TOML)、配置结构体(serde)、合并逻辑

  

### Phase 2: 网关核心 (9 个文件)

7. src/gateway/proxy.rs - 反向代理核心 (axum + hyper)

8. src/gateway/director.rs - 请求路由 (gRPC检测/黑名单/static-direct/WASM/PreHandler/Resolver)

9. src/gateway/transport.rs - 多协议 transport (HTTP/H2C/H3)

10. src/gateway/resolver.rs - 前端/后端路由解析

11. src/gateway/balancer.rs - 轮询/随机负载均衡

12. src/gateway/proxy_cache.rs - 代理目标缓存 (moka)

13. src/gateway/manager.rs - 多服务器生命周期管理

14. src/gateway/server/http_server.rs - HTTP/HTTPS 服务器

15. src/gateway/server/h3_server.rs - HTTP/3 服务器 (quinn)

    

### Phase 3: 中间件 + 流控 (12 个文件)

16. src/middleware/pre_handler/ - 认证/清理/域名控制/限流/图片保护

17. src/middleware/modifier/ - TraceID/安全头/CORS/Gzip/自定义头/错误响应

18. src/flow_control/ - 固定窗口/漏桶/令牌桶/滑动窗口

19. src/breaker/mod.rs - 熔断器 (per-domain bucket)

    

### Phase 4: 前后端代理 (7 个文件)

20. src/frontend/ - 静态文件服务 + 文件缓存 + SPA fallback

21. src/backend/ - HTTP后端/TCP代理/透明代理

22. src/static_direct/ - 静态直连服务器

    

### Phase 5: gRPC + 高级功能 (10 个文件)

23. proto/service.proto + build.rs - gRPC 服务定义

24. src/grpc/ - gRPC 服务器 + 服务实现 + HTTP→gRPC桥接 + gRPC-Web代理

25. src/wasm_plugin/ - WASM 插件系统 (wasmtime)

26. src/stat/ - 统计收集 + GeoIP

27. src/health_probe/ - 健康探测 (moka)

28. src/latency/ - 延迟测量

29. src/notifier/ - 邮件通知

30. src/error_page/ - 错误页面

    

### Phase 6: 应用组装 + CLI (4 个文件)

31. src/initialize/ - 优先级注册表

32. src/app/application.rs - PizzaApp 主结构体

33. src/app/lifecycle.rs - SIGINT/SIGTERM 优雅关闭

34. src/main.rs - clap CLI (run/generate/test/reload)