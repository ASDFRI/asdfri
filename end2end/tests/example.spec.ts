import { expect, test, type Page } from "@playwright/test";

const variants = {
  swiss: {
    label: "瑞士国际主义",
    themeColor: "#f3f0e9",
    background: "rgb(243, 240, 233)",
    colorScheme: "light",
  },
  constructivist: {
    label: "纯构成主义",
    themeColor: "#0b0b0b",
    background: "rgb(239, 230, 212)",
    colorScheme: "light",
  },
  epic: {
    label: "史诗叙事",
    themeColor: "#0a0a0e",
    background: "rgb(10, 10, 14)",
    colorScheme: "dark",
  },
} as const;

type Variant = keyof typeof variants;

const variantOrder = Object.keys(variants) as Variant[];

const isVariant = (value: string | null): value is Variant =>
  value !== null && Object.hasOwn(variants, value);

const expectVariantChrome = async (page: Page, variant: Variant) => {
  const constructivist = variant === "constructivist";
  const epic = variant === "epic";
  const agitation = page.locator(".agitation-strip");
  const manifesto = page.locator(".footer-manifesto");
  const overture = page.locator(".epic-overture");
  const saga = page.locator(".footer-epic");

  if (constructivist) {
    await expect(agitation).toBeVisible();
    await expect(manifesto).toBeVisible();
  } else {
    await expect(agitation).toHaveCount(0);
    await expect(manifesto).toHaveCount(0);
  }

  if (epic) {
    await expect(overture).toBeVisible();
    await expect(saga).toBeVisible();
  } else {
    await expect(overture).toHaveCount(0);
    await expect(saga).toHaveCount(0);
  }
};

test("首页渲染并展示内容", async ({ page }) => {
  await page.goto("/");

  await expect(page).toHaveTitle("ASDF Studio");
  await expect(page.locator("h1")).toContainText("我们创造");

  await expect(page.getByRole("link", { name: /Games/ }).first()).toBeVisible();
  await expect(page.getByText("蜂巢盛宴").first()).toBeVisible();
});

test("页面风格标记与条件内容一致", async ({ page }) => {
  await page.goto("/");

  const rawVariant = await page.locator("html").getAttribute("data-variant");
  expect(isVariant(rawVariant)).toBe(true);
  const variant: Variant = isVariant(rawVariant) ? rawVariant : "swiss";
  const meta = variants[variant];

  await expect(page.locator('meta[name="theme-color"]')).toHaveAttribute(
    "content",
    meta.themeColor,
  );
  await expect(page.locator('meta[name="color-scheme"]')).toHaveAttribute(
    "content",
    meta.colorScheme,
  );

  await expect
    .poll(() => page.evaluate(() => window.getComputedStyle(document.body).backgroundColor))
    .toBe(meta.background);

  await expectVariantChrome(page, variant);
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

    const rawInitial = await page.locator("html").getAttribute("data-variant");
    const initial: Variant = isVariant(rawInitial) ? rawInitial : "swiss";
    const target = variantOrder[(variantOrder.indexOf(initial) + 1) % variantOrder.length];
    const initialLabel = variants[initial].label;
    const targetLabel = variants[target].label;
    const targetCard = page.locator(".theme-option", { hasText: targetLabel });

    try {
      await expect(targetCard).toBeVisible();
      await targetCard.getByRole("button", { name: "切换到此风格" }).click();

      await expect(page.locator("html")).toHaveAttribute("data-variant", target);
      await expect(page.locator('meta[name="theme-color"]')).toHaveAttribute(
        "content",
        variants[target].themeColor,
      );
      await expect(page.locator('meta[name="color-scheme"]')).toHaveAttribute(
        "content",
        variants[target].colorScheme,
      );
      await expect
        .poll(() => page.evaluate(() => window.getComputedStyle(document.body).backgroundColor))
        .toBe(variants[target].background);
      await expect(targetCard.getByText("当前风格")).toBeVisible();

      await page.locator(".console-brand").click();
      await expect(page).toHaveURL(/\/$/);
      await expectVariantChrome(page, target);

      await page.goto("/console/theme");
      await page.waitForSelector('html[data-hydrated="true"]');
      await expect(page.locator("html")).toHaveAttribute("data-variant", target);

      await page.reload();
      await expect(page.locator("html")).toHaveAttribute("data-variant", target);

      await page.goto("/");
      await expectVariantChrome(page, target);
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
          variants[initial].themeColor,
        );
        await expect(page.locator('meta[name="color-scheme"]')).toHaveAttribute(
          "content",
          variants[initial].colorScheme,
        );
      }

      await page.goto("/");
      await expect(page.locator("html")).toHaveAttribute("data-variant", initial);
      await expectVariantChrome(page, initial);
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
