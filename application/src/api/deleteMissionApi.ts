import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";
import { baseURL } from "./baseURL";

export default async function deleteMissionApi(
  missionId: string,
): Promise<Result<null, ErrorCode>> {
  try {
    const res = await fetch(`${baseURL}/daily/${missionId}`, {
      method: "DELETE",
      credentials: "include",
    });
    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(
        `Delete Mission failed: ${errRes.message}(code: ${errRes.code})`,
      );

      return {
        ok: false,
        err: errRes.code,
      };
    }

    return {
      ok: true,
      value: null,
    };
  } catch (err) {
    console.error("Network error:", err);
    return {
      ok: false,
      err: -1,
    };
  }
}
