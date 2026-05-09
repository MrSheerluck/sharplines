export async function post<T>(
  _path: string,
  _body: unknown,
): Promise<{ data?: T; error?: string; status: number }> {
  return { status: 0, error: "not implemented" };
}

export async function get<T>(
  _path: string,
): Promise<{ data?: T; error?: string; status: number }> {
  return { status: 0, error: "not implemented" };
}
