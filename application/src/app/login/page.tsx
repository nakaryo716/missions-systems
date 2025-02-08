'use client'
import Login from "@/components/Login";
import React, { useState } from "react";
import { Login as LoginType } from "@/types/Login";
import loginApi from "@/api/loginApiHandler";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [err, SetErr] = useState<string | null>(null);
  const router = useRouter();

  const handleSetEmail = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setEmail(e.target.value);
  };

  const handleSetPassword = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setPassword(e.target.value);
  }

  const handleLogin = async () => {
    const loginPayload: LoginType = {
      email,
      password,
    };

    const res = await loginApi(loginPayload);
    if (res) {
      router.push("/");
    } else {
      SetErr("ログインに失敗しました");
    }
  };

  return (
    <Login
      emailVal={email}
      handleSetEmail={handleSetEmail}
      passwordVal={password}
      handleSetPassword={handleSetPassword}
      handleClickButton={handleLogin}
      errMsg={err}
    />
  )
}
