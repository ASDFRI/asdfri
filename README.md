# asdfri

ASDF Studio 官网与管理控制台。基于 Leptos + Axum 的 SSR Workspace，内置瑞士国际主义、纯构成主义与史诗叙事三套视觉体系，可在控制台动态切换。

## 视觉体系

- `swiss`：瑞士国际主义。严格网格、克制线条与明确层级；纸白、炭黑与单一正红。
- `constructivist`：纯构成主义。不对称网格、斜切色块、粗描边、硬阴影与宣传海报式排版。
- `epic`：史诗叙事。深空黑、鎏金细节、衬线大标题、电影化光影与章节式留白。
- 当前风格持久化在 `site_settings` 表，由 `/console/theme` 管理；写入后下一次服务端渲染生效。
- 页面根节点带有 `data-variant` 标记，样式以 `html[data-variant="..."]` 为作用域，便于自动化测试与样式定位。
- 样式入口为 `style/main.scss`，按设计令牌、基础排版、通用组件、站点框架、页面与响应式断点拆分为多个 SCSS 分片；纯构成主义与史诗叙事覆盖分别集中在 `style/_variant-constructivist.scss` 与 `style/_variant-epic.scss`。

## 架构

| Crate | 职责 | 编译目标 |
| --- | --- | --- |
| `domain` | 纯业务模型、输入清洗与领域校验 | 通用 |
| `infra` | 配置、数据库、JWT、会话、上传与日志 | 服务端 |
| `api` | 客户端与服务端共用的 Server Functions 与 DTO | 通用 |
| `ui` | 与业务无关的基础 UI 组件 | 通用 |
| `app` | 页面、业务组件、HTML 骨架、元信息与路由装配 | 通用 |
| `server` | 服务端二进制、中间件与 HTTP/HTTPS 监听 | 服务端 |
| `frontend` | wasm 水合入口 | 浏览器 |

依赖方向：

- `server` / `frontend` → `app`
- `app` → `ui`、`api`
- `api` → `infra`、`domain`
- `infra` → `domain`
- `ui` → `leptos`，不依赖 `api`、`infra`、`domain` 或任何业务状态
- `server` 同时直接依赖 `infra`

## 路由

| 路径 | 页面 |
| --- | --- |
| `/` | 首页：工作室、动态、作品、团队、招聘、联系 |
| `/technology` | 引擎、资产管线、创意、基础设施与美学 |
| `/trailer` | 年度预告片放映页 |
| `/games` | 六份作品档案 |
| `/singularis` | Singularis 框架架构、模块、工具链与入口 |
| `/console` | 控制台仪表盘 |
| `/console/hero` | 首页首屏内容槽位说明 |
| `/console/games` | 游戏档案 CRUD 与排序 |
| `/console/members` | 成员档案 CRUD 与排序 |
| `/console/theme` | 站点视觉风格切换（需要管理员授权） |

## 数据

- 数据库：SQLite，连接串见 `config.toml` 的 `[database].url`。
- 迁移：`migrations/`，启动时自动执行。
- 初始内容：迁移会写入四份作品与五名成员；管理员账户同样由迁移写入，其明文密码会在首次启动时自动升级为 Argon2 哈希。
- 站点风格：`site_settings` 单行表保存 `swiss`、`constructivist` 或 `epic`，默认 `swiss`；服务端在启动时载入缓存，授权写入成功后同步更新。
- 上传：写入 `[upload].dir`（默认 `uploads`），由 `/uploads/*` 显式提供；仅接受由魔数识别的 PNG、JPEG、GIF、WebP 与 AVIF，单文件上限 16 MiB。该目录独立于 `target/site`，重新构建不会丢失文件。

## 开发

```sh
cargo leptos watch
```

生产构建：

```sh
cargo leptos build --release
```

生产产物：

- `target/release/server`（Windows 为 `server.exe`）
- `target/release/hash.txt`（资源哈希清单，须与服务器二进制同目录）
- `target/site/`（静态资源、带哈希的 CSS、wasm 与 JS）

部署目录无需工作区 `Cargo.toml`，release 构建的监听地址由 `config.toml` 提供：

```toml
[server]
host = "0.0.0.0"
http_port = 80
https_port = 443
```

将 `server` 二进制、`hash.txt` 与 `target/site/` 一并分发，并在包含该目录的工作目录中运行。缺少 `hash.txt` 或站点产物与清单不一致时，服务端会拒绝启动并提示补全，避免 SSR 生成无效的资源地址。debug 与 release 共用 `target/site`，切换 profile 后须重新构建同一 profile 再启动。静态资源目录沿用 Leptos 官方约定（默认 `target/site`，可用 `LEPTOS_SITE_ROOT` 覆盖），不属于应用配置。开发构建（`cargo leptos watch`）使用 cargo-leptos 注入的 `LEPTOS_SITE_ADDR`，与 release 的 `[server]` 相互独立。

确保 `[database].url` 指向的目录与 `[upload].dir` 可写。release 模式下 `[auth].jwt_secret` 为空会拒绝启动。

## 校验

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
cargo leptos build

# 需要先启动服务，或使用 cargo leptos test
cd end2end && npx playwright test --project=chromium
```

## 配置

`config.toml` 在首次缺失时自动生成随机 JWT 密钥。若手动留空 `[auth].jwt_secret`，服务端会使用进程内临时密钥并在日志中告警，重启将使现有会话失效。

`[server]` 字段：

| 字段 | 说明 |
| --- | --- |
| `host` | 监听主机，须为 IP 字面量 |
| `http_port` | HTTP 端口；启用 TLS 时仅用于 HTTPS 重定向 |
| `https_port` | HTTPS 端口；仅在启用 TLS 时生效 |

`[server]` 仅作用于 release 构建；开发构建使用 cargo-leptos 注入的 `LEPTOS_SITE_ADDR`。

TLS 默认关闭。启用后 `http_port` 与 `https_port` 不能相同。本地 HTTPS 证书生成方式见 `self_signed_certs/README.md`。

## 缓存

构建产物启用 `hash-files`，JS、WASM 与 CSS 的文件名包含内容哈希，内容变化时 URL 随之变化，旧版本不会被复用。服务端按路径输出 `Cache-Control`：

| 路径 | `Cache-Control` | 说明 |
| --- | --- | --- |
| `/pkg/*` | `public, max-age=31536000, immutable` | 哈希文件名，可永久缓存 |
| `/pkg/snippets/*` | `no-cache` | wasm-bindgen 不哈希该目录 |
| `/uploads/*` | `public, max-age=31536000, immutable` | 文件名为 UUID，内容不可变 |
| `/api/*`、`/console`、`/console/*` | `no-store` | 按会话授权的内容 |
| SSR 页面 | `no-cache` | 每次使用前重新验证 |
| 其余静态资源 | `public, max-age=3600` | 图片、视频等，更新后最多延迟一小时 |

错误响应统一使用 `no-store`，避免 404/500 被缓存。`/pkg` 由独立的 `ServeDir` 提供，保留 `If-None-Match` 与 `If-Modified-Since`，静态文件未变化时返回 304。

## 安全

- 密码以 Argon2 哈希存储；迁移中的默认管理员明文密码会在首次启动时自动升级。
- 首次部署后请立即更换默认管理员凭据（更新数据库中的 Argon2 哈希）。
- 生产环境必须为 `[auth].jwt_secret` 填入随机字符串；release 模式下该值为空会导致服务端拒绝启动。
- 所有写操作在服务端再次校验 JWT；客户端状态不构成授权依据。
- 上传仅接受通过魔数识别的图片，单文件上限 16 MiB，文件名由服务端生成。
