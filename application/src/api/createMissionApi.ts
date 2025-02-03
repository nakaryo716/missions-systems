import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";

// ミッションの新規作成APIを呼ぶ
export default async function createMissionApi(payload: DailyMissionInput): Promise<Result<null, ErrorCode>> {
  try {
    // CookieとJson payloadをともにAPIを叩く
    const res = await fetch("http://localhost/api/daily", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
      credentials: "include",
    });
    // 作成に失敗した際はエラーコードを返して、
    // 呼び出し元にエラー処理を移譲する
    if (!res.ok) {
      const err: ApiError = await res.json();
      console.error(`Login failed:${err.message}(code: ${err.code})`);
      return {
        ok: false,
        err: err.code,
      };
    }
    // APIはレスポンスボディを返さないため
    // 成功した場合はnullを渡す
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
