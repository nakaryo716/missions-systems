import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";
import { baseURL } from "./baseURL";

// ユーザーを削除するAPI
export default async function deleteUserApi(): Promise<Result<null, ErrorCode>> {
  try {
    const res = await fetch(`${baseURL}/user`, {
      method: "DELETE",
      credentials: "include",
    });

    if (!res.ok) {
      const err: ApiError = await res.json();
      console.error(`Delete user failed:${err.message}(code: ${err.code})`);
      return {
        ok: false,
        err: err.code,
      };
    } else {
      return {
        ok: true,
        value: null,
      };
    }
  } catch (err) {
    console.error("Network error:", err);
    return {
      ok: false,
      err: -1,
    };
  }
}
