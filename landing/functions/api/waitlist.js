export async function onRequestPost(context) {
	const { request, env } = context;

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
		`CREATE TABLE IF NOT EXISTS waitlist (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			email TEXT UNIQUE,
			created_at TEXT DEFAULT (datetime('now'))
		)`
	);

	try {
		await env.sharplines_waitlist
			.prepare("INSERT INTO waitlist (email) VALUES (?)")
			.bind(email)
			.run();

		return Response.redirect(new URL(request.url).origin + "/?subscribed=true", 302);
	} catch (e) {
		if (e.message?.includes("UNIQUE constraint")) {
			return Response.redirect(new URL(request.url).origin + "/?already=true", 302);
		}
		return new Response(JSON.stringify({ error: "Something went wrong" }), {
			status: 500,
			headers: { "content-type": "application/json" },
		});
	}
}
