import { check } from "k6";
import http from "k6/http";

export let options = {
  vus: 50,
  duration: "30s",
};

export default function () {
  let res = http.get("http://localhost:8999/api/posts");
  check(res, {
    "status is 200": (r) => r.status === 200,
  });
}
