import "@wirunekaewjai/ts/htmx-interceptor/prelude";

import { $counter } from "@/client/views/counter";

// +/- on client-side
interceptor.add("/@counter", ({ query }) => {
  const count = Number(query.count);
  return $counter(count);
});