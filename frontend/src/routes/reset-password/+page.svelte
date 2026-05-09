<script lang="ts">
    import { onMount } from "svelte";
    import { post } from "$lib/api/client";
    let token = $state<string | null>(null);
    let password = $state("");
    let error = $state("");
    let loading = $state(false);
    let success = $state(false);
    onMount(() => {
        if (typeof window !== "undefined") {
            token = new URLSearchParams(window.location.search).get("token");
        }
    });
    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        error = "";
        if (!password) {
            error = "Password is required";
            return;
        }
        if (password.length < 8) {
            error = "Password must be at least 8 characters";
            return;
        }
        loading = true;
        try {
            const result = await post<{}>("/auth/reset-password", {
                token,
                password,
            });
            if (result.error) throw new Error(result.error);
            success = true;
        } catch (err) {
            error = err instanceof Error ? err.message : "Reset failed";
        } finally {
            loading = false;
        }
    }
</script>

<div class="flex-grow flex items-center justify-center p-6 z-10">
    <div class="w-full max-w-[440px]">
        <div
            class="bg-surface-container border border-outline-variant rounded overflow-hidden"
        >
            <div class="h-1 w-full bg-primary-container"></div>
            <div class="p-8 space-y-6">
                {#if !token}
                    <p role="alert" class="text-error text-center">
                        Invalid or missing reset token
                    </p>
                    <div class="text-center">
                        <a
                            href="/forgot-password"
                            class="text-sm text-primary hover:underline"
                            >Request a new reset link</a
                        >
                    </div>
                {:else if success}
                    <p class="text-secondary font-semibold text-center">
                        Password updated! You can now log in.
                    </p>
                    <div class="text-center">
                        <a
                            href="/login"
                            class="text-sm text-primary hover:underline"
                            >Go to login</a
                        >
                    </div>
                {:else}
                    <div class="space-y-2 text-center">
                        <h1
                            class="text-3xl font-bold text-on-surface tracking-tight"
                        >
                            Set New Password
                        </h1>
                        <p class="text-on-surface-variant">
                            Enter your new password
                        </p>
                    </div>
                    <form onsubmit={handleSubmit} class="space-y-5">
                        <div class="space-y-2">
                            <label
                                class="text-xs text-outline uppercase tracking-widest font-mono"
                                for="password">New Password</label
                            >
                            <input
                                id="password"
                                type="password"
                                bind:value={password}
                                class="w-full bg-surface-dim border border-outline-variant focus:border-primary-container text-on-surface px-4 py-2.5 rounded font-mono placeholder:text-outline-variant transition-colors outline-none"
                                placeholder="••••••••"
                            />
                        </div>
                        {#if error}
                            <p role="alert" class="text-error text-sm">
                                {error}
                            </p>
                        {/if}
                        <button
                            type="submit"
                            disabled={loading}
                            class="w-full border border-primary-container text-primary hover:bg-primary-container hover:text-on-primary-container font-semibold py-3 rounded transition-all disabled:opacity-50"
                        >
                            {loading ? "Resetting…" : "Reset Password"}
                        </button>
                    </form>
                {/if}
            </div>
        </div>
    </div>
</div>
