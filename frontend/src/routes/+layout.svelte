<script lang="ts">
    import "../app.css";
    import favicon from "$lib/assets/favicon.svg";
    import logo from "$lib/assets/logo.svg";
    import { getUser, getAccessToken, logout } from "$lib/stores/auth.svelte";
    let { children } = $props();
    let user = $derived(getUser());
    let token = $derived(getAccessToken());
    let isAuth = $derived(user !== null && token !== null);
    async function handleLogout() {
        await logout();
    }
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>
<div
    class="chess-grid-overlay opacity-10 pointer-events-none fixed inset-0"
></div>
<header class="w-full border-b border-outline-variant bg-surface relative z-10">
    <div
        class="flex justify-between items-center px-6 py-4 w-full max-w-[1440px] mx-auto"
    >
        <a href="/" class="flex items-center gap-2 no-underline">
            <img src={logo} alt="SharpLines" class="h-8 w-auto" />
            <span class="text-xl font-bold text-primary">SharpLines</span>
        </a>
        <nav class="flex items-center gap-4">
            {#if isAuth}
                <span class="text-sm text-on-surface-variant"
                    >{user!.display_name || user!.email}</span
                >
                <button
                    onclick={handleLogout}
                    class="text-sm text-primary hover:underline bg-transparent border-none cursor-pointer"
                    >Log Out</button
                >
            {:else}
                <a
                    href="/login"
                    class="text-sm text-on-surface-variant hover:text-primary transition-colors"
                    >Log In</a
                >
                <a
                    href="/register"
                    class="text-sm bg-primary-container text-on-primary-container px-3 py-1.5 rounded font-semibold hover:opacity-90 transition-opacity no-underline"
                    >Register</a
                >
            {/if}
        </nav>
    </div>
</header>
<main class="flex-grow flex relative z-10">
    {@render children()}
</main>
<footer
    class="w-full border-t border-outline-variant bg-surface-container relative z-10"
>
    <div class="px-6 py-6 w-full max-w-[1440px] mx-auto">
        <span class="text-xs font-bold text-primary uppercase">SharpLines</span>
        <p class="text-xs text-on-surface-variant mt-1">
            Built for serious chess players.
        </p>
    </div>
</footer>

<style>
    :global(body) {
        background-color: var(--color-surface);
        font-family: "Inter", ui-sans-serif, system-ui, sans-serif;
        margin: 0;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
    }
    :global(body.base) {
        background-color: var(--color-surface);
    }
    .chess-grid-overlay {
        background-size: 40px 40px;
        background-image:
            linear-gradient(to right, #1e293b 1px, transparent 1px),
            linear-gradient(to bottom, #1e293b 1px, transparent 1px);
    }
</style>
