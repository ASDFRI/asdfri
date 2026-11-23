-- 首页首屏内容表：每个视觉变体一行
CREATE TABLE IF NOT EXISTS hero_content (
    variant         TEXT PRIMARY KEY,             -- 视觉变体：swiss / constructivist
    eyebrow         TEXT NOT NULL DEFAULT '',     -- 眉题
    location        TEXT NOT NULL DEFAULT '',     -- 地点标记
    title           TEXT NOT NULL DEFAULT '[]',   -- 主标题行（JSON 数组）
    lede            TEXT NOT NULL DEFAULT '',     -- 引导文本
    primary_label   TEXT NOT NULL DEFAULT '',     -- 主操作文案
    primary_href    TEXT NOT NULL DEFAULT '',     -- 主操作链接
    secondary_label TEXT NOT NULL DEFAULT '',     -- 次操作文案
    secondary_href  TEXT NOT NULL DEFAULT '',     -- 次操作链接
    media_tag       TEXT NOT NULL DEFAULT '',     -- 媒体角标
    media_poster    TEXT NOT NULL DEFAULT '',     -- 视频封面
    media_video     TEXT NOT NULL DEFAULT '',     -- 视频地址
    strip           TEXT NOT NULL DEFAULT '[]'    -- 底部滚动标签（JSON 数组）
);
