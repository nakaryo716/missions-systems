import { Result } from "@/types/Result";
import { ErrorCode } from "@/types/ErrorCode";
import { ApiError } from "@/types/ApiError";

export default async function missionCompleteApi(missionId: string): Promise<Result<null, ErrorCode>>{
  try {
    const res = await fetch(`http://localhost/api/daily/complete/${missionId}`, {
      method: "PUT",
      credentials: "include",
    });

    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(`Signup Failed: ${errRes.message}(code: ${errRes.code})`);
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
