<script lang="ts">
    import { post } from "$lib/api/client";
    let email = $state("");
    let error = $state("");
    let loading = $state(false);
    let sent = $state(false);
    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        error = "";
        if (!email.trim()) {
            error = "Email is required";
            return;
        }
        loading = true;
        try {
            const result = await post<{}>("/auth/forgot-password", { email });
            if (result.error) throw new Error(result.error);
            sent = true;
        } catch (err) {
            error = err instanceof Error ? err.message : "Request failed";
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
                <div class="space-y-2 text-center">
                    <h1
                        class="text-3xl font-bold text-on-surface tracking-tight"
                    >
                        Reset Password
                    </h1>
                    <p class="text-on-surface-variant">
                        Enter your email to receive a reset link
                    </p>
                </div>
                {#if sent}
                    <p class="text-secondary font-semibold text-center">
                        Check your email for the reset link
                    </p>
                    <div class="text-center">
                        <a
                            href="/login"
                            class="text-sm text-primary hover:underline"
                            >Return to login</a
                        >
                    </div>
                {:else}
                    <form onsubmit={handleSubmit} class="space-y-5">
                        <div class="space-y-2">
                            <label
                                class="text-xs text-outline uppercase tracking-widest font-mono"
                                for="email">Email</label
                            >
                            <input
                                id="email"
                                type="email"
                                bind:value={email}
                                class="w-full bg-surface-dim border border-outline-variant focus:border-primary-container text-on-surface px-4 py-2.5 rounded font-mono placeholder:text-outline-variant transition-colors outline-none"
                                placeholder="email@address.com"
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
                            {loading ? "Sending…" : "Send Reset Link"}
                        </button>
                    </form>
                    <div class="text-center">
                        <a
                            href="/login"
                            class="text-sm text-primary hover:underline"
                            >Back to login</a
                        >
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
