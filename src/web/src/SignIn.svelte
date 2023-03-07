<script>
  import Logo from "./Logo.svelte";
  import OutlinedInput from "./Components/OutlinedInput.svelte";
</script>

<div
  class="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8"
>
  <div
    class="dark:bg-gradient-radial-0 w-full max-w-md space-y-8 rounded-3xl p-8 shadow-lg"
  >
    <header>
      <Logo class="mx-auto h-28 w-auto" />
      <h2
        class="mt-6 text-center text-3xl font-medium text-gray-900 dark:text-emphasis-high"
      >
        Sign in to your account
      </h2>
    </header>
    <form class="space-y-6 px-5" action="#" method="POST">
      <OutlinedInput
        id="username"
        name="username"
        label="Username"
        autocomplete="username"
        required
      />
      <OutlinedInput
        id="password"
        name="password"
        label="Password"
        type="password"
        autocomplete="current-password"
        required
      />

      <div>
        <button
          type="submit"
          class="group relative flex w-full justify-center rounded-full border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-gray-50 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 dark:text-emphasis-high dark:hover:bg-indigo-500"
        >
          Sign in
          <div class="absolute inset-y-0 right-0 flex items-center pr-4">
            <svg
              class="h-6 w-6"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M15.75 9V5.25A2.25 2.25 0 0013.5 3h-6a2.25 2.25 0 00-2.25 2.25v13.5A2.25 2.25 0 007.5 21h6a2.25 2.25 0 002.25-2.25V15M12 9l-3 3m0 0l3 3m-3-3h12.75"
              />
            </svg>
          </div>
        </button>
      </div>
    </form>
  </div>
</div>
