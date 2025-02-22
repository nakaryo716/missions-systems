import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";

export default async function updateMissionApi(
  missionId: string,
  mission: DailyMissionInput,
): Promise<Result<null, ErrorCode>> {
  try {
    const res = await fetch(`http://localhost/api/daily/${missionId}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify(mission),
    });
    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(
        `Update Mission failed: ${errRes.message}(code: ${errRes.code})`,
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
