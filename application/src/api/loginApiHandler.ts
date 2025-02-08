import { ApiError } from "@/types/ApiError";
import { Login } from "@/types/Login";

export default async function loginApi(payload: Login): Promise<boolean> {    
  try {
    // call login api
    const res = await fetch("http://localhost/api/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });
    
    if (!res.ok) {
      const err: ApiError = await res.json();
      console.error(`Login failed:${err.message}(code: ${err.code})`);
      return false;
    }
    return true;
  } catch (err) {
    console.error("Network Error", err);
    return false;
  }
}
