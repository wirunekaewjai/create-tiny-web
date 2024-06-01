import { counter } from "@/client/views/counter";
import { hxGet } from "@wirunekaewjai/jetpack";

// +/- on client-side
hxGet((path, query) => {
  if (path === "/@counter") {
    const count = Number(query.get("count"));
    return counter(count);
  }
});