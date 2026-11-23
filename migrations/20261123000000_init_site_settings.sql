-- 站点设置表：单行记录，控制公开站点的视觉变体
CREATE TABLE IF NOT EXISTS site_settings (
    id      INTEGER PRIMARY KEY CHECK (id = 1), -- 固定单行
    variant TEXT    NOT NULL DEFAULT 'swiss'    -- 视觉变体：swiss / constructivist
);

INSERT OR IGNORE INTO site_settings (id, variant) VALUES (1, 'swiss');
