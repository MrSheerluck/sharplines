<script lang="ts">
    import { login } from "$lib/stores/auth.svelte";

    let email = $state("");
    let password = $state("");
    let error = $state("");
    let loading = $state(false);

    async function handleSubmit(e: SubmitEvent) {
        e.preventDefault();
        if (!email.trim()) {
            error = "Email is required";
            loading = false;
            return;
        }
        if (!password) {
            error = "Password is required";
            loading = false;
            return;
        }
        error = "";
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

<h1>Log In</h1>

<form onsubmit={handleSubmit}>
    <label>
        Email
        <input type="email" bind:value={email} />
    </label>
    <label>
        Password
        <input type="password" bind:value={password} />
    </label>
    <button type="submit" disabled={loading}>
        {loading ? "Logging in..." : "Log In"}
    </button>
</form>
<p>
    Don't have an account? <a href="/register">Register</a>
</p>
<hr />
<a href="http://localhost:3000/auth/lichess">Login with Lichess</a>
<a href="/forgot-password">Forgot password?</a>
{#if error}
    <p role="alert">{error}</p>
{/if}
