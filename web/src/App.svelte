<script>
  import Logo from "./Logo.svelte";
</script>

<main class="">
  <div
    class="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8"
  >
    <div class="w-full max-w-md space-y-8 p-8 rounded-3xl dark:bg-gradient-radial-0 shadow-lg">
      <div>
        <Logo class="mx-auto h-28 w-auto" />
        <h2
          class="mt-6 text-center text-3xl font-medium text-gray-900 dark:text-emphasis-high"
        >
          Sign in to your account
        </h2>
      </div>
      <form class="mt-8 space-y-6" action="#" method="POST">
        <input type="hidden" name="remember" value="true" />
        <div class="-space-y-px rounded-md shadow-sm">
          <div>
            <label for="email-address" class="sr-only">Email address</label>
            <input
              id="email-address"
              name="email"
              type="email"
              autocomplete="email"
              required
              class="relative block w-full appearance-none rounded-none rounded-t-md border border-gray-300 dark:border-gray-500 px-3 py-2 text-gray-900 dark:text-emphasis-high placeholder-gray-500 dark:placeholder-gray-300 focus:z-10 focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
              placeholder="Email address"
            />
          </div>
          <div>
            <label for="password" class="sr-only">Password</label>
            <input
              id="password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              class="relative block w-full appearance-none rounded-none rounded-b-md border border-gray-300 dark:border-gray-500 px-3 py-2 text-gray-900 dark:text-emphasis-high placeholder-gray-500 dark:placeholder-gray-300 focus:z-10 focus:border-indigo-500 focus:dark:border-indigo-300 focus:outline-none focus:ring-indigo-500 focus:dark:ring-indigo-300  sm:text-sm"
              placeholder="Password"
            />
          </div>
        </div>

        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <input
              id="remember-me"
              name="remember-me"
              type="checkbox"
              class="h-4 w-4 rounded border-gray-300 text-indigo-600 dark:text-indigo-400 focus:ring-indigo-500 dark:focus:ring-indigo-400"
            />
            <label
              for="remember-me"
              class="ml-2 block text-sm text-gray-900 dark:text-emphasis-medium"
              >Remember me</label
            >
          </div>

          <div class="text-sm">
            <a
              href="#"
              class="font-medium text-indigo-600 dark:text-indigo-400 hover:text-indigo-500 dark:hover:text-indigo-300"
              >Forgot your password?</a
            >
          </div>
        </div>

        <div>
          <button
            type="submit"
            class="group relative flex w-full justify-center rounded-full border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium dark:text-emphasis-high text-gray-50 hover:bg-indigo-700 dark:hover:bg-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
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
</main>
