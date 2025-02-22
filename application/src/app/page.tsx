'use client'
import createMissionApi from "@/api/createMissionApi";
import getMissionApi from "@/api/getMissionApi";
import { DailyMission } from "@/types/DailyMission";
import App from "@/components/App";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { levelApi } from "@/api/levelApi";
import missionCompleteApi from "@/api/missionCompleteApi";

export default function Root() {
  const router = useRouter();
  const [loading, setLoading] = useState(true);

  // 初回ローディング時にAPIを叩いて、Missionを取得しsetする
  // 取得できなかった場合はログインページにリダイレクトする
  // TODO:認証してない場合は/login、サーバーなどの問題のときはエラーメッセージを表示する
  const [missions, setMissions] = useState<DailyMission[]>([]);
  const [userStatus, setUserStatus] = useState<null | Level>(null);

  useEffect(() => {
    const handleGetMissions = async () => {
      const res = await getMissionApi();
      if (!res.ok) {
        router.push("/login");
        return;
      } else {
        setMissions(res.value);
        setLoading(false);
        return;
      }
    };

    const handleGetStatus = async () => {
      const res = await levelApi();
      if (!res.ok) {
        router.push("/login");
        return;
      } else {
        setUserStatus(res.value);
        setLoading(false);
        return;
      }
    }

    handleGetMissions();
    handleGetStatus();
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

  // ミッションが追加された時に再度取得するためのハンドラ
  const handleGetMissions = async () => {
    const res = await getMissionApi();
    if (!res.ok) {
      router.push("/login");
    } else {
      setMissions(res.value);
    }
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
    // TODO: ミッションは最大7個まで登録できる
    //       8個以上になったらエラーメッセージを表示させる
    if (!res.ok) {
      setErr("ミッションの追加に失敗しました");
      console.error(err);
    }
    // 初期化
    setMissionTitle("");
    setMissionDescription(null);
    // Missionを再度取得する
    handleGetMissions();
    return;
  }

  const handleGetStatus = async () => {
    const res = await levelApi();
    if (!res.ok) {
      alert("通信に失敗しました");
      return;
    } else {
      setUserStatus(res.value);
      return;
    }
  }

  // ミッションをコンプリートにするハンドラ
  const handleCompleteMission = async (id: string) => {
    const res = await missionCompleteApi(id);
    if (!res.ok) {
      alert("通信に失敗しました");
    }
    // 再度ミッションとレベルを取得する
    handleGetMissions();
    handleGetStatus();
  };

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
          missions={missions}
          userStatus={userStatus}
          handleComplete={handleCompleteMission}
          handleGetMission={handleGetMissions}
        />
      }
    </>
  )
}
