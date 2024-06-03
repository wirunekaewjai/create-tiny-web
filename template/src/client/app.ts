import "./prelude";

import { counter } from "@/client/views/counter";
import { parseHxQuery } from "@jetpack/functions/parse-hx-query";
import { sendHxResponse } from "@jetpack/functions/send-hx-response";

const onAfterSwap = (e: CustomEvent) => {
  const detail = e.detail as {
    elt: HTMLElement;
    pathInfo: {
      requestPath: string;
    };
  };

  const [pathname] = detail.pathInfo.requestPath.split("?");

  // bind icon name in svg element for preserve svg
  if (pathname.startsWith("/icons/")) {
    return detail.elt.setAttribute("name", pathname.split(".")[0].split("/")[2]);
  }
};

const onBeforeRequest = (e: CustomEvent) => {
  const xhr = e.detail.xhr as XMLHttpRequest;
  const conf = e.detail.requestConfig as {
    path: string;
    parameters: Record<string, any>;
  };

  const [pathname, search] = conf.path.split("?");

  if (pathname === "/@count") {
    e.preventDefault();

    const query = parseHxQuery(search, conf.parameters) as {
      name: string;
      value: number;
    };

    return sendHxResponse(xhr, {
      body: counter(query.name, query.value),
      url: conf.path,
    });
  }
};

htmx.on("htmx:afterSwap", (e) => {
  return onAfterSwap(e as CustomEvent);
});

// wrapped async function with normal function because htmx not supported async function
htmx.on("htmx:beforeRequest", (e) => {
  return onBeforeRequest(e as CustomEvent);
});