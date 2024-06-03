export function icon(name: string) {
  const svg = document.querySelector(`svg[name="${name}"]`);
  return svg?.outerHTML ?? "";
}
