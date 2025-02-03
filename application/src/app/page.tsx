'use client'
import createMissionApi from "@/api/createMissionApi";
import getUserInfoApi from "@/api/userInfoApi";
import App from "@/components/App";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Root() {
  const router = useRouter();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    getUserInfoApi().then(res => {
      if (!res.ok) {
        console.error(res.err);
        router.push("/login");
        return;
      }
      console.log(res.value);
      setLoading(false);
      return;
    })
  }, [router]);

  // ミッション追加の実装
  const [missionTitle, setMissionTitle] = useState("");
  const [missionDescription, setMissionDescription] = useState<null | string>(null);
  const [err, setErr] = useState<null | string>(null);
  
  const handleSetTitle = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setMissionTitle(e.target.value);
  };

  const handleSetDescription = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    setMissionDescription(e.target.value);
  };

  // ミッションを追加するためのハンドラ
  const handleAddMission = async () => {
    // ペイロードの作成
    const payload: DailyMissionInput = {
      title: missionTitle,
      description: missionDescription,
    };
    // API call
    const res = await createMissionApi(payload);

    if (!res.ok) {
      setErr("ミッションの追加に失敗しました");
      console.error(err);
    }
    // 初期化
    setMissionTitle("");
    setMissionDescription(null);
    return;
  }

  return(
    <>
      {
        loading ? 
          <div style={{textAlign: "center", alignItems: "center"}}>
            <h1>Now Loading...</h1>
          </div>
        : <App 
          title={missionTitle}
          setTitle={handleSetTitle}
          description={missionDescription}
          setDescription={handleSetDescription}
          submitHandle={handleAddMission}
        />
      }
    </>
  )
}
