export function parseHxQuery<T>(search: string | URLSearchParams, hxVals?: Record<string, string | number | boolean>) {
  const query: Record<string, string | number | boolean> = { ...hxVals };

  new URLSearchParams(search).forEach((value, key) => {
    const boo = value.toUpperCase();

    if (boo === "TRUE" || boo === "FALSE") {
      query[key] = (boo === "TRUE");
    } else {
      const num = Number(value);
      query[key] = (Number.isNaN(num) ? value : num);
    }
  });

  return query as T;
}
