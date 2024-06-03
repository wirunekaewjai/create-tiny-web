export function icon(name: string) {
  name += ".svg";

  const svg = document.querySelector(`svg[name="${name}"]`);

  if (svg) {
    // using loaded svg instead of reload
    return svg.outerHTML;
  }

  const path = `/icons/${name}?v=1`;

  return (
    <svg
      hx-get={path}
      hx-trigger="load"
      hx-swap="outerHTML"
    />
  );
}
