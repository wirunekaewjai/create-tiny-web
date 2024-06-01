import { hxGet } from "@wirunekaewjai/jetpack";
import { counter } from "@/client/views/counter";

// +/- on client-side
hxGet((path, query) => {
  if (path === "/@counter") {
    const count = Number(query.get("count"));
    return counter(count);
  }
});