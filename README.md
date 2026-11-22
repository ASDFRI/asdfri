# asdfri

ASDF Studio 官网与管理控制台 · `pure-constructivism` 分支。基于 Leptos + Axum 的 SSR Workspace，采用纯构成主义视觉：红、黑、米白、斜切几何、硬阴影与宣传海报式排版。

## 视觉分支

- `main`：构成主义与瑞士国际主义混合版本。
- `pure-constructivism`：纯构成主义版本。仅使用红、黑、米白三色；强调斜切色块、硬阴影、密集排版、宣传条与宣言式页脚。
- 页面根节点带有 `data-variant="constructivist"` 标记，便于自动化测试与样式定位。

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

## 数据

- 数据库：SQLite，连接串见 `config.toml` 的 `[database].url`。
- 迁移：`migrations/`，启动时自动执行。
- 初始内容：迁移会写入四份作品与五名成员；管理员账户同样由迁移写入，其明文密码会在首次启动时自动升级为 Argon2 哈希。
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
- `target/site/`（静态资源、`pkg/asdfri.css`、wasm 与 JS）

在缺少工作区 `Cargo.toml` 的部署目录中，通过环境变量提供运行时配置：

```sh
LEPTOS_OUTPUT_NAME=asdfri
LEPTOS_SITE_ROOT=/srv/asdfri/site
LEPTOS_SITE_PKG_DIR=pkg
LEPTOS_SITE_ADDR=0.0.0.0:3000
APP_CONFIG=/srv/asdfri/config.toml
RUST_LOG=info
```

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

TLS 默认关闭。本地 HTTPS 证书生成方式见 `self_signed_certs/README.md`。

## 安全

- 密码以 Argon2 哈希存储；迁移中的默认管理员明文密码会在首次启动时自动升级。
- 首次部署后请立即更换默认管理员凭据（更新数据库中的 Argon2 哈希）。
- 生产环境必须为 `[auth].jwt_secret` 填入随机字符串；release 模式下该值为空会导致服务端拒绝启动。
- 所有写操作在服务端再次校验 JWT；客户端状态不构成授权依据。
- 上传仅接受通过魔数识别的图片，单文件上限 16 MiB，文件名由服务端生成。
