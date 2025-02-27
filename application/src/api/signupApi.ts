import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";

export default async function signupApi(payload: CreateUser): Promise<[boolean, ErrorCode]> {
  // call api
  try {
    const res = await fetch("http://localhost/api/user", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    // error handling
    // return false and error code
    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(`Signup Failed: ${errRes.message}(code: ${errRes.code})`);
      return [false, errRes.code];
    }
    return [true, null];
  } catch (err) {
    console.error("Network Error:", err);
    return [false, -1];
  }
}
