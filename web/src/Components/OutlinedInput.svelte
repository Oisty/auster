<script lang="ts">
  import { createEventDispatcher } from "svelte";

  // Parameters
  export let assistiveText: string | null = null;
  export let isError: boolean = false;
  export let value: string | null = null;

  // Computed styles
  const marginBottom = assistiveText ? "" : "mb-6";
  const borderColor = isError
    ? "border-red-300"
    : "group-focus-within:border-primary-100 group-hover:group-focus-within:border-primary-100 border-emphasis-low group-hover:border-emphasis-medium";

  $: [translate, fontSize, borderTop, borderHoverColor] = !value
    ? ["translate-y-1/3", "", "", ""]
    : [
        "-translate-y-3.5",
        "text-xs",
        "border-t-transparent",
        "group-hover:border-t-transparent",
      ];

  const determineTextColor = () => {
    if (isError) return "text-red-300";
    if (/*!isError &&*/ !value)
      return "text-emphasis-medium group-focus-within:text-primary-100";

    return "text-emphasis-low group-focus-within:text-primary-100";
  };

  const textColor = determineTextColor();

  // Events
  const dispatch = createEventDispatcher();

  function handleValueChanged(event: Event) {
    value = (<HTMLInputElement>event.target).value;
    dispatch("change", value);
  }
</script>

<div>
  <div
    class="focus-within:border-primary-100 group relative rounded-md {marginBottom}"
  >
    <input
      class="w-full touch-manipulation rounded-md py-3 px-4
            leading-none outline-none transition-all 
            placeholder:text-lg placeholder:opacity-0 placeholder:transition-all autofill:bg-black placeholder:focus:opacity-100 
            dark:bg-elevation-2 dark:text-emphasis-high placeholder:dark:text-emphasis-medium
            hover:dark:bg-elevation-1 focus:dark:bg-elevation-0"
      value={$$props.value ?? null}
      id={$$props.id}
      placeholder={$$props.placeholder ?? " "}
      on:change={handleValueChanged}
    />

    <div
      class="pointer-events-none absolute top-0 left-0 flex h-full w-full origin-top-left touch-manipulation"
    >
      <!--Filler start-->
      <div class="w-4 {borderColor} rounded-l-md border-y border-l" />

      <!--Filler middle-->
      <div
        class="border-y {borderColor} {borderHoverColor} px-0.5 group-focus-within:border-t-transparent group-hover:group-focus-within:border-t-transparent group-focus-within:group-hover:border-t-transparent {borderTop}"
      >
        <div
          class="transition-all group-focus-within:-translate-y-3.5 {translate}"
        >
          <label
            class="{textColor} {fontSize} whitespace-nowrap group-focus-within:text-xs"
            for={$$props.id}>{$$props.label}</label
          >
        </div>
      </div>

      <!--Filler end-->
      <div class="w-full border-y border-r {borderColor} rounded-r-md" />
    </div>
  </div>
  <!--Assistive text-->
  {#if isError && assistiveText}
    <span class="pl-4 text-xs text-red-300">{assistiveText}</span>
  {:else if !isError && assistiveText}
    <span class="pl-4 text-xs text-emphasis-medium">{assistiveText}</span>
  {/if}
</div>
