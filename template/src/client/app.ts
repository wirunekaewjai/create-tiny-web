import { counter } from "@/client/views/counter";
import { hxResponse } from "@wirunekaewjai/jetpack";

htmx.on("htmx:afterSwap", (e) => {
  const event = e as CustomEvent;
  const detail = event.detail as {
    elt: HTMLElement;
    pathInfo: {
      requestPath: string;
    };
  };

  const [pathname] = detail.pathInfo.requestPath.split("?");

  // bind icon name in svg element for preserve svg
  if (pathname.startsWith("/icons/")) {
    return detail.elt.setAttribute("name", pathname.split("/")[2]);
  }
});

htmx.on("htmx:beforeRequest", (e) => {
  const event = e as CustomEvent;
  const conf = event.detail.requestConfig as {
    path: string;
    parameters: Record<string, any>;
  };

  const [pathname, search] = conf.path.split("?");

  if (pathname === "/@count") {
    const hxVals = conf.parameters;
    const query = new URLSearchParams(search);

    const name = query.get("name") ?? hxVals.name;
    const value = Number(query.get("value") ?? hxVals.value);

    return hxResponse(event, {
      body: counter(name, value),
      url: conf.path,
    });
  }
});