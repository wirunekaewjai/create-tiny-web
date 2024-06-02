import { icon } from "@/client/views/icon";

export function counter(name: string, value: number) {
  return (
    <div
      class="p-2 flex flex-row items-center"
      id={name}
      hx-swap="outerHTML"
      hx-swap-oob="true"
    >
      <button
        class="w-8 h-8 bg-red-600 text-white rounded-md shadow-md fill-current flex items-center justify-center p-2"
        hx-get={`/@count?name=${name}&value=${value - 1}`}
      >
        {icon("fa-solid-minus")}
      </button>
      <div class="flex items-center px-4 h-8 mx-2 border rounded-md">
        {value}
      </div>
      <button
        class="w-8 h-8 bg-blue-600 text-white rounded-md shadow-md fill-current flex items-center justify-center p-2"
        hx-get={`/@count?name=${name}&value=${value + 1}`}
      >
        {icon("fa-solid-plus")}
      </button>
    </div>
  );
}
