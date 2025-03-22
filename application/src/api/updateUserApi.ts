import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";
import { baseURL } from "./baseURL";

// ユーザー名を変更するAPI
// クエリパラメータで変更
export default async function updateUserApi(
  name: string
): Promise<Result<null, ErrorCode>> {
  try {
    const res = await fetch(`${baseURL}/user?user_name=${name}`, {
      method: "PUT",
      credentials: "include",
    });

    if (!res.ok) {
      const err: ApiError = await res.json();
      console.error(`Update user failed:${err.message}(code: ${err.code})`);
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
