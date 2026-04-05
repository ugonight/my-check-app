<script lang="ts">
  import { authUser, logout, isAuthenticated } from "$lib/stores/auth";
  import { goto } from "$app/navigation";

  let isOpen = $state(false);

  const handleLogout = async () => {
    await logout();
    isOpen = false;
    goto("/auth/login");
  };

  const handleClickOutside = (e: MouseEvent) => {
    const target = e.target as HTMLElement;
    if (
      target.closest("[data-offcanvas-trigger]") ||
      target.closest("[data-offcanvas-content]")
    ) {
      return;
    }
    isOpen = false;
  };
</script>

<svelte:document onclick={handleClickOutside} />

<!-- Account Icon Button - Fixed at bottom-left -->
<button
  data-offcanvas-trigger
  onclick={() => (isOpen = !isOpen)}
  class="fixed bottom-4 left-4 w-12 h-12 rounded-full bg-blue-500 hover:bg-blue-600 transition-colors flex items-center justify-center text-white z-40 shadow-lg"
  title="アカウント"
>
  <!-- Google Icons "person" -->
  <svg
    xmlns="http://www.w3.org/2000/svg"
    height="24px"
    viewBox="0 -960 960 960"
    width="24px"
    fill="#e3e3e3"
    ><path
      d="M367-527q-47-47-47-113t47-113q47-47 113-47t113 47q47 47 47 113t-47 113q-47 47-113 47t-113-47ZM160-160v-112q0-34 17.5-62.5T224-378q62-31 126-46.5T480-440q66 0 130 15.5T736-378q29 15 46.5 43.5T800-272v112H160Zm80-80h480v-32q0-11-5.5-20T700-306q-54-27-109-40.5T480-360q-56 0-111 13.5T260-306q-9 5-14.5 14t-5.5 20v32Zm296.5-343.5Q560-607 560-640t-23.5-56.5Q513-720 480-720t-56.5 23.5Q400-673 400-640t23.5 56.5Q447-560 480-560t56.5-23.5ZM480-640Zm0 400Z"
    /></svg
  >
</button>

<!-- Offcanvas Overlay -->
{#if isOpen}
  <div
    class="fixed inset-0 bg-black/30 dark:bg-black/50 z-30 transition-opacity"
  ></div>
{/if}

<!-- Offcanvas Panel -->
<div
  data-offcanvas-content
  class={`fixed top-0 left-0 h-full w-64 bg-white dark:bg-neutral-950 border-r border-neutral-200 dark:border-neutral-800 z-40 transition-transform duration-200 ${
    isOpen ? "translate-x-0" : "-translate-x-full"
  }`}
>
  <div class="p-6">
    <h2 class="text-lg font-bold mb-6 text-neutral-900 dark:text-neutral-50">
      アカウント
    </h2>

    {#if $isAuthenticated && $authUser}
      <div class="space-y-4">
        <div class="p-3 bg-neutral-100 dark:bg-neutral-800 rounded-md">
          <p class="text-xs text-neutral-600 dark:text-neutral-400 mb-1">
            ログイン中
          </p>
          <p
            class="text-sm font-medium text-neutral-900 dark:text-neutral-50 break-all"
          >
            {$authUser.email}
          </p>
        </div>

        <button
          onclick={handleLogout}
          class="w-full px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors text-sm font-medium"
        >
          ログアウト
        </button>
      </div>
    {:else}
      <a
        href="/auth/login"
        onclick={() => (isOpen = false)}
        class="w-full block px-4 py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 transition-colors text-sm font-medium text-center"
      >
        ログイン
      </a>
    {/if}
  </div>
</div>
