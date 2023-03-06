import { test, expect } from "@playwright/experimental-ct-svelte";
import SignIn from "../src/SignIn.svelte";

const usernameInputSelector = "input[autocomplete=username]";
const passwordInputSelector = "input[autocomplete=current-password]";

test("Sign in has username and password input", async ({ mount }) => {
  // Arrange
  const component = await mount(SignIn);
  const usernameInput = await component
    .locator(usernameInputSelector)
    .elementHandle();

  const passwordInput = await component
    .locator(passwordInputSelector)
    .elementHandle();

  // Assert
  expect(usernameInput).not.toBeNull();
  expect(passwordInput).not.toBeNull();
});

test("Sign in component password input should have password type attribute", async ({
  mount,
}) => {
  // Arrange
  const component = await mount(SignIn);
  const typeAttribute = await component
    .locator(passwordInputSelector)
    .getAttribute("type");

  // Assert
  expect(typeAttribute).toBe("password");
});

test("Sign in component username input should be required", async ({
  mount,
}) => {
  // Arrange
  const component = await mount(SignIn);

  const requiredAttribute = await component
    .locator(usernameInputSelector)
    .getAttribute("required");

  // Assert
  // It's an empty attribute that just has to be there as a flag
  expect(requiredAttribute).toBe("");
});

test("Sign in component password input should be required", async ({
  mount,
}) => {
  // Arrange
  const component = await mount(SignIn);

  const requiredAttribute = await component
    .locator(passwordInputSelector)
    .getAttribute("required");

  // Assert
  // It's an empty attribute that just has to be there as a flag
  expect(requiredAttribute).toBe("");
});
