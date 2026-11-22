-- 游戏作品表
CREATE TABLE IF NOT EXISTS game (
    id          INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增主键
    name        TEXT    NOT NULL UNIQUE,           -- 作品名称（唯一）
    screenshots TEXT    NOT NULL DEFAULT '[]',     -- 截图地址（JSON 数组）
    description TEXT,                              -- 作品简介
    tags        TEXT    NOT NULL DEFAULT '[]',     -- 分类标签（JSON 数组）
    link        TEXT,                              -- 外部链接
    sort_order  INTEGER NOT NULL DEFAULT 0         -- 展示排序
);

CREATE INDEX IF NOT EXISTS idx_game_sort_order ON game(sort_order);

-- 团队成员表
CREATE TABLE IF NOT EXISTS member (
    id          INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增主键
    name        TEXT    NOT NULL UNIQUE,           -- 成员姓名（唯一）
    icon        TEXT,                              -- 头像地址
    description TEXT,                              -- 详细描述
    slogan      TEXT,                              -- 个性签名
    tags        TEXT    NOT NULL DEFAULT '[]',     -- 领域标签（JSON 数组）
    position    TEXT,                              -- 组织职位
    sort_order  INTEGER NOT NULL DEFAULT 0         -- 展示排序
);

CREATE INDEX IF NOT EXISTS idx_member_sort_order ON member(sort_order);

-- 初始作品档案
INSERT OR IGNORE INTO game (name, screenshots, description, tags, link, sort_order) VALUES
    ('蜂巢盛宴', '["/Games/HiveFeast.jpg"]',
     '在蜂群崩塌之前，重建属于你的巢穴。以极简的规则编织复杂的生态，让每一次采集与分配都成为秩序的选择。',
     '["蜂群","生存","模拟"]', NULL, 0),
    ('回家的路', '["/Games/TheWayHome.jpeg"]',
     '土路是广阔的游乐场，也是孤独的牢笼。一部关于等待、记忆与鄉愁的交互式叙事作品。',
     '["叙事","共情","乡村"]', NULL, 1),
    ('逃离酒店', '["/Games/EscapeHotel.jpg"]',
     '走廊没有尽头，房间永远温热。在阈限空间中保持视线接触，寻找一条不存在的出口。',
     '["心理恐怖","阈限空间","解谜"]', NULL, 2),
    ('月之暗面', '["/Games/TheDarksideOfTheMoon.jpg"]',
     '血肉已逝，精神在硅与星光中延续。于月球的寂静尘埃上，见证一次不属于人类的创世。',
     '["科幻","月球","AI"]', NULL, 3);

-- 初始成员档案
INSERT OR IGNORE INTO member (name, icon, description, slogan, tags, position, sort_order) VALUES
    ('TrifingZW', '/Teams/TrifingZW.jpg',
     '负责整体技术方向与引擎架构，关注有序的复杂系统与克制的表达。',
     '把每一行代码当作结构的一部分。', '["Rust","架构","引擎"]', '技术总监', 0),
    ('GS', '/Teams/GS.png',
     '负责玩法系统与关卡节奏，相信规则本身就是叙事。',
     '先让系统自洽，再让世界呼吸。', '["玩法","系统","关卡"]', '游戏设计', 1),
    ('Egg', '/Teams/Egg.jpg',
     '负责概念艺术与视觉识别，在几何与留白之间建立张力。',
     '少即是多，空白也是内容。', '["概念艺术","视觉"]', '艺术总监', 2),
    ('ElysiaGwen', '/Teams/ElysiaGwen.png',
     '负责角色与叙事文本，将情绪压缩进最短的句子。',
     '故事从沉默开始。', '["叙事","角色"]', '叙事设计', 3),
    ('Pg', '/Teams/Pg.jpg',
     '负责云基础设施与构建流水线，让发布成为可重复的事实。',
     '稳定不是偶然，是设计。', '["Axum","运维","CI"]', '基础设施', 4);
