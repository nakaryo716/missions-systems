import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";
import { DailyMission } from "@/types/DailyMission";

export default async function getMissionApi(): Promise<Result<DailyMission[], ErrorCode>> {
  try {
    const res = await fetch("http://localhost/api/daily", {
      method: "GET",
      credentials: "include",
    });
    // 作成に失敗した際はエラーコードを返して、
    // 呼び出し元にエラー処理を移譲する
    if (!res.ok) {
      const err: ApiError = await res.json();
      console.error(`Get missions failed:${err.message}(code: ${err.code})`);
      return {
        ok: false,
        err: err.code,
      };
    }
    // レスポンスボディのパース
    // DailyMissionの配列で渡される
    const missions: DailyMission[] = await res.json();
    return {
      ok: true,
      value: missions,
    };
  } catch (err) {
    console.error("Network error:", err);
    return {
      ok: false,
      err: -1,
    };
  }
}
