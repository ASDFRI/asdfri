import { expect, test } from "@playwright/test";

test("首页渲染并展示内容", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("ASDF Studio");
  await expect(page.locator("h1")).toContainText("我们创造");

  await expect(page.getByRole("link", { name: /Games/ }).first()).toBeVisible();
  await expect(page.getByText("蜂巢盛宴").first()).toBeVisible();
});

test("纯构成主义分支标记", async ({ page }) => {
  await page.goto("/");

  await expect(page.locator("html")).toHaveAttribute("data-variant", "constructivist");
  await expect(page.locator(".agitation-strip")).toBeVisible();
  await expect(page.locator(".footer-manifesto")).toBeVisible();
});

test("作品页与 Singularis 页可访问", async ({ page }) => {
  await page.goto("/games");
  await expect(page).toHaveTitle("Games — ASDF Studio");
  await expect(page.getByRole("heading", { name: /六个/ })).toBeVisible();

  await page.goto("/singularis");
  await expect(page).toHaveTitle("Singularis — ASDF Studio");
  await expect(page.getByRole("heading", { name: "引力奇点" })).toBeVisible();
});

test("控制台展示登录入口", async ({ page }) => {
  await page.goto("/console");

  await expect(page).toHaveTitle("Dashboard — ASDF Console");
  await expect(page.getByRole("button", { name: "登录" })).toBeVisible();
  await expect(page.getByRole("heading", { name: "仪表盘" })).toBeVisible();
});

test("管理员登录与编辑对话框回填", async ({ page }) => {
  await page.goto("/console");
  await page.waitForSelector('html[data-hydrated="true"]');
  await page.getByRole("button", { name: "登录" }).click();

  const login = page.getByRole("dialog", { name: "登录控制台" });
  await expect(login).toBeVisible();
  await login.getByLabel("用户名").fill("admin");
  await login.getByLabel("密码").fill("admin123");
  await login.getByRole("button", { name: "登录" }).click();

  await expect(page.getByText("已认证")).toBeVisible();

  await page.goto("/console/games");
  await page.waitForSelector('html[data-hydrated="true"]');
  await expect(page.getByRole("heading", { name: "游戏配置" })).toBeVisible();
  await page.getByRole("button", { name: "编辑" }).first().click();

  const editor = page.getByRole("dialog", { name: "游戏档案" });
  await expect(editor).toBeVisible();
  await expect(editor.getByLabel("游戏名称 *")).toHaveValue("蜂巢盛宴");
  await editor.getByRole("button", { name: "取消" }).click();
  await expect(editor).toBeHidden();
});

test("团队成员与作品图片可加载", async ({ page }) => {
  await page.goto("/");

  await page.locator("#team").scrollIntoViewIfNeeded();
  const portrait = page.locator(".person__media img").first();
  await expect
    .poll(async () => portrait.evaluate((image: HTMLImageElement) => image.naturalWidth))
    .toBeGreaterThan(0);

  await page.locator("#games").scrollIntoViewIfNeeded();
  const cover = page.locator(".dossier__media img").first();
  await expect
    .poll(async () => cover.evaluate((image: HTMLImageElement) => image.naturalWidth))
    .toBeGreaterThan(0);
});

test("移动端菜单可导航并自动关闭", async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto("/");
  await page.waitForSelector('html[data-hydrated="true"]');

  await page.locator(".nav-mobile__toggle").click();
  const panel = page.locator(".nav-mobile__panel");
  await expect(panel).toBeVisible();

  await panel.getByRole("link", { name: /Games/ }).click();
  await expect(page).toHaveURL(/\/games$/);
  await expect(panel).toHaveCount(0);
});

test("管理员可上传截图资产", async ({ page }) => {
  await page.goto("/console");
  await page.waitForSelector('html[data-hydrated="true"]');
  await page.getByRole("button", { name: "登录" }).click();

  const login = page.getByRole("dialog", { name: "登录控制台" });
  await login.getByLabel("用户名").fill("admin");
  await login.getByLabel("密码").fill("admin123");
  await login.getByRole("button", { name: "登录" }).click();
  await expect(page.getByText("已认证")).toBeVisible();

  await page.goto("/console/games");
  await page.waitForSelector('html[data-hydrated="true"]');
  await page.getByRole("button", { name: "编辑" }).first().click();

  const editor = page.getByRole("dialog", { name: "游戏档案" });
  await expect(editor).toBeVisible();
  await expect(editor.locator(".upload__item")).toHaveCount(1);

  await editor.locator('input[type="file"]').setInputFiles("../public/ASDF.png");
  await expect(page.getByText("截图上传成功")).toBeVisible();
  await expect(editor.locator(".upload__item")).toHaveCount(2);

  await editor.getByRole("button", { name: "取消" }).click();
  await expect(editor).toBeHidden();
});
