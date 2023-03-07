import SignIn from "../src/SignIn.svelte";
import { beforeAll, afterAll, test, it, describe } from "vitest";
import type { PreviewServer } from "vite";
import type { Browser, Page } from "playwright";
import { expect } from "@playwright/test";
import { preview } from "vite";
import { chromium } from "playwright";

describe("Sign in", () => {
  const usernameInputSelector = "input[autocomplete=username]";
  const passwordInputSelector = "input[autocomplete=current-password]";

  // Tests should not not depend on each other, so avoid implicitly passed state like this
  let page: Page;
  beforeAll(async () => {
    const port = 3000;
    const server = await preview({ preview: { port } });
    const browser = await chromium.launch();
    page = await browser.newPage();
    await page.goto(`http://localhost:${port}/signin`);

    return async () => {
      await browser.close();
      await new Promise<void>((resolve, reject) => {
        server.httpServer.close((error) => (error ? reject(error) : resolve()));
      });
    };
  });

  it("Should have a username and password input", async () => {
    // Arrange
    const usernameInput = await page
      .locator(usernameInputSelector)
      .elementHandle();

    const passwordInput = await page
      .locator(passwordInputSelector)
      .elementHandle();

    // Assert
    expect(usernameInput).not.toBeNull();
    expect(passwordInput).not.toBeNull();
  });

  it("Should have a password input with the password type attribute", async () => {
    // Arrange
    const typeAttribute = await page
      .locator(passwordInputSelector)
      .getAttribute("type");

    // Assert
    expect(typeAttribute).toBe("password");
  });

  it("Should have a username input that is required", async () => {
    // Arrange

    const requiredAttribute = await page
      .locator(usernameInputSelector)
      .getAttribute("required");

    // Assert
    // It's an empty attribute that just has to be there as a flag
    expect(requiredAttribute).toBe("");
  });

  it("Should have a password input that is required", async () => {
    // Arrange
    const requiredAttribute = await page
      .locator(passwordInputSelector)
      .getAttribute("required");

    // Assert
    // It's an empty attribute that just has to be there as a flag
    expect(requiredAttribute).toBe("");
  });

  it.todo("Should have inputs with the same name as the server expexts");

  it.todo("Should have a form that posts to the sign in enpoint");
});
