export async function onRequestPost(context) {
	try {
		const { request, env } = context;

		if (!env.sharplines_waitlist) {
			return new Response(
				JSON.stringify({ error: "D1 binding not configured. Add 'sharplines_waitlist' in Pages → Settings → Functions → D1 Database Bindings." }),
				{ status: 500, headers: { "content-type": "application/json" } }
			);
		}

		let email;
		const contentType = request.headers.get("content-type") || "";

		if (contentType.includes("application/json")) {
			const body = await request.json();
			email = body.email;
		} else {
			const formData = await request.formData();
			email = formData.get("email");
		}

		if (!email) {
			return new Response(JSON.stringify({ error: "Email is required" }), {
				status: 400,
				headers: { "content-type": "application/json" },
			});
		}

		await env.sharplines_waitlist.exec(
			"CREATE TABLE IF NOT EXISTS waitlist (id INTEGER PRIMARY KEY AUTOINCREMENT, email TEXT UNIQUE, created_at TEXT DEFAULT (datetime('now')))"
		);

		await env.sharplines_waitlist
			.prepare("INSERT INTO waitlist (email) VALUES (?)")
			.bind(email)
			.run();

		return Response.redirect(new URL(request.url).origin + "/?subscribed=true", 302);
	} catch (e) {
		return new Response(
			JSON.stringify({ error: e.message || "Unknown error" }),
			{ status: 500, headers: { "content-type": "application/json" } }
		);
	}
}
