import { expect, test } from "@playwright/test";

test("首页渲染并展示内容", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("ASDF Studio");
  await expect(page.locator("h1")).toContainText("我们创造");

  await expect(page.getByRole("link", { name: /Games/ }).first()).toBeVisible();
  await expect(page.getByText("蜂巢盛宴").first()).toBeVisible();
});

test("页面风格标记与条件内容一致", async ({ page }) => {
  await page.goto("/");

  const variant = await page.locator("html").getAttribute("data-variant");
  expect(["swiss", "constructivist"]).toContain(variant as string);

  await expect(page.locator('meta[name="theme-color"]')).toHaveAttribute(
    "content",
    variant === "constructivist" ? "#0b0b0b" : "#f3f0e9",
  );

  await expect
    .poll(() => page.evaluate(() => window.getComputedStyle(document.body).backgroundColor))
    .toBe(variant === "constructivist" ? "rgb(239, 230, 212)" : "rgb(243, 240, 233)");

  if (variant === "constructivist") {
    await expect(page.locator(".agitation-strip")).toBeVisible();
    await expect(page.locator(".footer-manifesto")).toBeVisible();
  } else {
    await expect(page.locator(".agitation-strip")).toHaveCount(0);
    await expect(page.locator(".footer-manifesto")).toHaveCount(0);
  }
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

test.describe("站点风格切换", () => {
  test.skip(({ browserName }) => browserName !== "chromium", "全局风格切换在单一浏览器中验证");

  test("管理员可切换视觉变体并持久化", async ({ page }) => {
    await page.goto("/console/theme");
    await page.waitForSelector('html[data-hydrated="true"]');
    await page.getByRole("button", { name: "登录" }).click();

    const login = page.getByRole("dialog", { name: "登录控制台" });
    await login.getByLabel("用户名").fill("admin");
    await login.getByLabel("密码").fill("admin123");
    await login.getByRole("button", { name: "登录" }).click();
    await expect(page.getByText("已认证")).toBeVisible();

    const initial = (await page.locator("html").getAttribute("data-variant")) ?? "swiss";
    const target = initial === "constructivist" ? "swiss" : "constructivist";
    const initialLabel = initial === "constructivist" ? "纯构成主义" : "瑞士国际主义";
    const targetLabel = target === "constructivist" ? "纯构成主义" : "瑞士国际主义";
    const targetCard = page.locator(".theme-option", { hasText: targetLabel });

    try {
      await expect(targetCard).toBeVisible();
      await targetCard.getByRole("button", { name: "切换到此风格" }).click();

      await expect(page.locator("html")).toHaveAttribute("data-variant", target);
      await expect(page.locator('meta[name="theme-color"]')).toHaveAttribute(
        "content",
        target === "constructivist" ? "#0b0b0b" : "#f3f0e9",
      );
      await expect
        .poll(() => page.evaluate(() => window.getComputedStyle(document.body).backgroundColor))
        .toBe(target === "constructivist" ? "rgb(239, 230, 212)" : "rgb(243, 240, 233)");
      await expect(targetCard.getByText("当前风格")).toBeVisible();

      await page.locator(".console-brand").click();
      await expect(page).toHaveURL(/\/$/);
      if (target === "constructivist") {
        await expect(page.locator(".agitation-strip")).toBeVisible();
        await expect(page.locator(".footer-manifesto")).toBeVisible();
      } else {
        await expect(page.locator(".agitation-strip")).toHaveCount(0);
        await expect(page.locator(".footer-manifesto")).toHaveCount(0);
      }

      await page.goto("/console/theme");
      await page.waitForSelector('html[data-hydrated="true"]');
      await expect(page.locator("html")).toHaveAttribute("data-variant", target);

      await page.reload();
      await expect(page.locator("html")).toHaveAttribute("data-variant", target);

      await page.goto("/");
      if (target === "constructivist") {
        await expect(page.locator(".agitation-strip")).toBeVisible();
        await expect(page.locator(".footer-manifesto")).toBeVisible();
      } else {
        await expect(page.locator(".agitation-strip")).toHaveCount(0);
        await expect(page.locator(".footer-manifesto")).toHaveCount(0);
      }
    } finally {
      await page.goto("/console/theme");
      await page.waitForSelector('html[data-hydrated="true"]');
      await expect(page.getByText("已认证")).toBeVisible();

      const initialCard = page.locator(".theme-option", { hasText: initialLabel });
      await expect(initialCard).toBeVisible();

      const switchBack = initialCard.getByRole("button", { name: "切换到此风格" });
      if (await switchBack.count()) {
        await switchBack.click();
        await expect(page.locator("html")).toHaveAttribute("data-variant", initial);
        await expect(page.locator('meta[name="theme-color"]')).toHaveAttribute(
          "content",
          initial === "constructivist" ? "#0b0b0b" : "#f3f0e9",
        );
      }

      await page.goto("/");
      await expect(page.locator("html")).toHaveAttribute("data-variant", initial);
    }
  });
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
