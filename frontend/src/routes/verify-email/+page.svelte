<script lang="ts">
    import { onMount } from "svelte";
    import { get } from "$lib/api/client";
    let status = $state<"loading" | "success" | "error">("loading");
    let message = $state("");
    function getTokenFromUrl(): string | null {
        if (typeof window === "undefined") return null;
        const params = new URLSearchParams(window.location.search);
        return params.get("token");
    }
    onMount(async () => {
        const token = getTokenFromUrl();
        if (!token) {
            status = "error";
            message = "Missing verification token";
            return;
        }
        status = "loading";
        const result = await get<{}>(
            "/auth/verify-email?token=" + encodeURIComponent(token),
        );
        if (result.error) {
            status = "error";
            message = result.error;
        } else {
            status = "success";
            message = "Email verified! Redirecting to login…";
        }
    });
</script>

<div class="flex-grow flex items-center justify-center p-6 z-10">
    <div class="w-full max-w-[440px] text-center space-y-4">
        {#if status === "loading"}
            <p class="text-on-surface-variant">Verifying your email…</p>
        {:else if status === "success"}
            <p class="text-secondary font-semibold">{message}</p>
            <a href="/login" class="text-sm text-primary hover:underline"
                >Go to login</a
            >
        {:else}
            <p role="alert" class="text-error">{message}</p>
            <a href="/login" class="text-sm text-primary hover:underline"
                >Return to login</a
            >
        {/if}
    </div>
</div>
