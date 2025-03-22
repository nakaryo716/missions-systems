import { ApiError } from "@/types/ApiError";
import { ErrorCode } from "@/types/ErrorCode";
import { Result } from "@/types/Result";
import { UserInfo } from "@/types/UserInfo";
import { baseURL } from "./baseURL";

// Cookieに保存されているJWTをサーバーに渡してユーザー情報{userId, userName}を取得する
export default async function getUserInfoApi(): Promise<Result<UserInfo, ErrorCode>> {
  try {
    // Cookieをサーバーに渡す
    const res = await fetch(`${baseURL}/user`, {
      method: "GET",
      credentials: "include",
    });

    // 認証エラーorサーバーサイドのエラー
    if (!res.ok) {
      const errRes: ApiError = await res.json();
      console.error(`Get user information failed: ${errRes.message}(code: ${errRes.code})`);
      // return type = Err<ErrorCode>
      return {
        ok: false,
        err: errRes.code,
      };
    }
    // ユーザー情報のパース
    const userInfo: UserInfo = await res.json();
    // return type = Ok<UserInfo>
    return {
      ok: true,
      value: userInfo,
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
