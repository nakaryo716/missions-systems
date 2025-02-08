'use client'

import loginApi from "@/api/loginApiHandler";
import signupApi from "@/api/signupApi";
import SignUp from "@/components/Signup";
import { Login } from "@/types/Login";
import { useRouter } from "next/navigation";
import React, { useState } from "react";

export default function SignUpPage() {
  const [userName, setUserName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [err, setErr] = useState<string | null>(null);
  const router = useRouter();

  const handleSetUserName = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ): void => {
    setUserName(e.target.value);
  };

  const handleSetEmail = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ): void => {
    setEmail(e.target.value);
  };

  const handleSetPassword = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ): void => {
    setPassword(e.target.value);
  };

  const signupApiCall = async () => {
    setErr(null);
    // ユーザー作成のペイロード生成
    const payload: CreateUser = {
      userName,
      email,
      password,
    };

    const [ success, errCode ] = await signupApi(payload);

    // サインアップが成功したら、自動ログインしてHomeに移動する
    if (success) {
      // ログインペイロードの生成
      const payload: Login = {
        email,
        password,
      };
      
      const success = await loginApi(payload)
      // ログインが成功したらHomeに移動
      if (success) {
        router.push("/");
      //　 失敗したらログインページに飛ぶ
      // TODO:エラーメッセージを出す
      } else {
        router.push("/login");
      }
    } 

    // サインアップが失敗するとたどり着く
    // サインアップページに失敗したことを伝えるためのReact.Dispatch
    if (errCode) {
      // TODO: ここでエラーコードを元に適切なエラーメッセージに割り当てる
      setErr(errCode.toString());
    } else {
      setErr("予期せぬエラーが発生しました");
    }
  };

  return (
    <SignUp
      userName={userName}
      handleSetUserName={handleSetUserName}
      email={email}
      handleSetEmail={handleSetEmail}
      password={password}
      handleSetPassword={handleSetPassword}
      signupHandle={signupApiCall}
      err={err}
    />
  );
}
