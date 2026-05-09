<script lang="ts">
    import { getUser, getAccessToken } from "$lib/stores/auth.svelte";
    let { children } = $props();
    let isAuth = $derived(getUser() !== null && getAccessToken() !== null);
    $effect(() => {
        if (!isAuth && typeof window !== "undefined") {
            window.location.href = "/login";
        }
    });
</script>

{#if isAuth}
    {@render children()}
{/if}
