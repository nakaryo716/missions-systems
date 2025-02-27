'use client'
import getUserInfoApi from "@/api/userInfoApi";
import { UserInfo } from "@/types/UserInfo";
import { useRouter } from "next/navigation";
import React, { useEffect, useState } from "react";
import { MyPage as Info } from "@/components/MyPage";
import Header from "@/components/Header";
import style from "@/styles/App.module.css";
import updateUserApi from "@/api/updateUserApi";
import deleteUserApi from "@/api/deleteUserApi";

export default function MyPage() {
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);
  // ユーザ名を変更するためのstate
  const [userName, setUserName] = useState("");
  const handleSetUserName = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>): void => {
    setUserName(e.target.value);
    return;
  }
  // ユーザー名を変更するハンドラ
  // 変更できた際はリロードする
  const handleUpdate = async () => {
    if (userName.length === 0) {
      return;
    }

    const res = await updateUserApi(userName);
    if (!res.ok) {
      alert("更新に失敗しました");
      return;
    }
    window.location.reload();
    return;
  };
  // ユーザーを完全に削除するハンドラ
  // 削除後はサインアップ画面にリダイレクトする
  const handleDelete = async () => {
    const res = await deleteUserApi();
    if (!res.ok) {
      alert("ユーザーの削除に失敗しました");
    }

    router.push("/signup");
    return;
  };
  // 初回ロード時にユーザー情報(ユーザー名)を取得する
  useEffect(() => {
    getUserInfoApi().then(res => {
      if (!res.ok) {
        console.error(res.err);
        router.push("/login");
        return;
      }
      setUserInfo(res.value)
      setLoading(false);
      return;
    });
    return;
  }, [router]);
  
  return(
    <>
      <div>
        <div className={style.header}>
          <Header />
        </div>
        <div className={style.content}>
          {
            loading ? 
              <div style={{textAlign: "center", alignItems: "center"}}>
                <h1>Now Loading...</h1>
              </div>
            : <Info
                userInfo={userInfo}
                userName={userName}
                handleChangeName={handleSetUserName}
                handleUpdateName={handleUpdate}
                handleDelete={handleDelete}
              />
          }
        </div>
      </div>
    </>
  );
}
