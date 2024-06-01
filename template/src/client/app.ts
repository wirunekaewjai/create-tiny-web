import { counter } from "@/client/views/counter";
import { onHxGet } from "@wirunekaewjai/jetpack";

// +/- on client-side
onHxGet((path, query) => {
  if (path === "/@counter") {
    const count = Number(query.get("count"));
    return counter(count);
  }
});