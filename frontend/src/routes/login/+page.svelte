<script lang="ts">
    import logo from "$lib/assets/logo.svg";
    import { login } from "$lib/stores/auth.svelte";
    let email = $state("");
    let password = $state("");
    let error = $state("");
    let loading = $state(false);
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
        loading = true;
        try {
            await login(email, password);
        } catch (err) {
            error = err instanceof Error ? err.message : "Login failed";
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
                </div>
                <div class="space-y-4">
                    <a
                        href="http://localhost:3000/auth/lichess"
                        class="w-full bg-primary-container hover:opacity-90 text-on-primary-container font-semibold py-3 px-5 rounded flex items-center justify-center gap-3 transition-all"
                    >
                        <span>Login with Lichess</span>
                    </a>
                    <div class="flex items-center gap-4 py-2">
                        <div class="h-px flex-grow bg-outline-variant"></div>
                        <span
                            class="text-xs text-outline uppercase tracking-widest font-mono"
                            >or</span
                        >
                        <div class="h-px flex-grow bg-outline-variant"></div>
                    </div>
                </div>
                <form onsubmit={handleSubmit} class="space-y-5">
                    <div class="space-y-2">
                        <label
                            class="text-xs text-outline uppercase tracking-widest font-mono"
                            for="email">Email</label
                        >
                        <div class="relative">
                            <input
                                id="email"
                                type="email"
                                bind:value={email}
                                class="w-full bg-surface-dim border border-outline-variant focus:border-primary-container text-on-surface px-4 py-2.5 rounded font-mono placeholder:text-outline-variant transition-colors outline-none"
                                placeholder="email@address.com"
                            />
                        </div>
                    </div>
                    <div class="space-y-2">
                        <div class="flex justify-between items-baseline">
                            <label
                                class="text-xs text-outline uppercase tracking-widest font-mono"
                                for="password">Password</label
                            >
                            <a
                                href="/forgot-password"
                                class="text-xs text-primary hover:underline"
                                >Forgot Password?</a
                            >
                        </div>
                        <div class="relative">
                            <input
                                id="password"
                                type="password"
                                bind:value={password}
                                class="w-full bg-surface-dim border border-outline-variant focus:border-primary-container text-on-surface px-4 py-2.5 rounded font-mono placeholder:text-outline-variant transition-colors outline-none"
                                placeholder="••••••••"
                            />
                        </div>
                    </div>
                    {#if error}
                        <p role="alert" class="text-error text-sm">{error}</p>
                    {/if}
                    <button
                        type="submit"
                        disabled={loading}
                        class="w-full border border-primary-container text-primary hover:bg-primary-container hover:text-on-primary-container font-semibold py-3 rounded transition-all disabled:opacity-50"
                    >
                        {loading ? "Logging In" : "Login"}
                    </button>
                </form>
                <div class="text-center">
                    <p class="text-sm text-on-surface-variant">
                        New to the SharpLines?
                        <a
                            href="/register"
                            class="text-secondary font-bold hover:underline ml-1"
                            >Create an account</a
                        >
                    </p>
                </div>
            </div>
        </div>
    </div>
</div>
