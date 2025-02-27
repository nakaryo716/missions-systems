import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";

export async function levelApi(): Promise<Result<Level, ErrorCode>> {
  try {
    const res = await fetch("http://localhost/api/exp", {
      method: "GET",
      credentials: "include",
    });

    // 認証エラーorサーバーサイドのエラー
    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(`Get user level failed: ${errRes.message}(code: ${errRes.code})`);
      // return type = Err<ErrorCode>
      return {
        ok: false,
        err: errRes.code,
      };
    }
    // レスポンスボディをパース
    const level: Level = await res.json();
    return {
      ok: true,
      value: level,
    };
  } catch (err) {
    // ネットワークエラー時
    console.error("Network error:", err);
    // return type = Err<ErrorCode>
    return {
      ok: false,
      err: -1,
    };
  }
}
