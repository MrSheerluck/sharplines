<script lang="ts">
    import { register } from "$lib/stores/auth.svelte";
    import logo from "$lib/assets/logo.svg";
    let email = $state("");
    let password = $state("");
    let displayName = $state("");
    let error = $state("");
    let loading = $state(false);
    let success = $state(false);
    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        error = "";
        if (!email.trim()) {
            error = "Email is required";
            return;
        }
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
            await register(email, password, displayName || undefined);
            success = true;
        } catch (err) {
            error = err instanceof Error ? err.message : "Registration failed";
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
            <div class="p-8 space-y-6">
                <div class="space-y-2 text-center">
                    <div class="flex items-center justify-center gap-3">
                        <img src={logo} alt="SharpLines" class="h-10 w-auto" />
                        <h1
                            class="text-3xl font-bold text-on-surface tracking-tight"
                        >
                            SharpLines
                        </h1>
                    </div>
                    <p class="text-on-surface-variant">Create your account</p>
                </div>
                {#if success}
                    <div class="text-center space-y-4">
                        <p class="text-secondary font-semibold">
                            Check your email to verify your account
                        </p>
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
                        <div class="space-y-2">
                            <label
                                class="text-xs text-outline uppercase tracking-widest font-mono"
                                for="displayName">Display Name</label
                            >
                            <input
                                id="displayName"
                                type="text"
                                bind:value={displayName}
                                class="w-full bg-surface-dim border border-outline-variant focus:border-primary-container text-on-surface px-4 py-2.5 rounded font-mono placeholder:text-outline-variant transition-colors outline-none"
                                placeholder="ChessMaster"
                            />
                        </div>
                        <div class="space-y-2">
                            <label
                                class="text-xs text-outline uppercase tracking-widest font-mono"
                                for="password">Password</label
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
                            {loading ? "Registering…" : "Register"}
                        </button>
                    </form>
                    <div class="text-center">
                        <p class="text-sm text-on-surface-variant">
                            Already have an account?
                            <a
                                href="/login"
                                class="text-secondary font-bold hover:underline ml-1"
                                >Log In</a
                            >
                        </p>
                    </div>
                {/if}
            </div>
        </div>
    </div>
</div>
