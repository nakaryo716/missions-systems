'use client'
import { useState } from "react";
import style from "@/styles/App.module.css";
import Header from '@/components/Header';
import Selector from "@/components/Selector";
import MissionsList from "@/components/MissionsList";
import AddMission from "@/components/AddMission";
import Status from "@/components/Status"

type AppProps = {
  // ミッション追加のProps
  title: string,
  setTitle: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  description: null | string;
  setDescription: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  submitHandle: () => Promise<void>;
}
const App = (props: AppProps) => {
  const {
    title,
    setTitle,
    description,
    setDescription,
    submitHandle,
  } = props;

  const [selectorVal, setSelectorVal] = useState(0);

  const handleSelectorChange = (_e: React.SyntheticEvent<Element, Event>, newValue: number) => {
    setSelectorVal(newValue);
  };

  const mission: DailyMission = {
    userId: "hi",
    missionId: "1",
    title: "本を読む",
    description: "技術書をよむ",
    isComplete: true,
  };

  const mission2: DailyMission = {
    userId: "hi",
    missionId: "2",
    title: "ランニング",
    description: "2km走る",
    isComplete: false,
  };

  const mission3: DailyMission = {
    userId: "hi",
    missionId: "3",
    title: "ランニング",
    description: "2km走る",
    isComplete: false,
  };
  const mission4: DailyMission = {
    userId: "hi",
    missionId: "4",
    title: "ランニング",
    description: "2km走る",
    isComplete: false,
  }; 
   const mission5: DailyMission = {
    userId: "hi",
    missionId: "5",
    title: "ランニング",
    description: "2km走る",
    isComplete: false,
  };


  const missions = [mission, mission2, mission3, mission4, mission5];

  let selectedComponent;
  switch (selectorVal) {
    case 0:
      selectedComponent = <MissionsList missions={missions} />;
      break;
    case 1:
      selectedComponent = <AddMission 
        title={title}
        setTitle={setTitle}
        description={description}
        setDescription={setDescription}
        submitHandle={submitHandle}
      />;
      break;
    case 2:
      selectedComponent = <Status />;
      break;
  };

  return (
    <div className={style.app}>
      <div className={style.header}>
        <Header/>
      </div>
      <div className={style.selector}>
        <Selector value={selectorVal} handleChange={handleSelectorChange}/>
      </div>
      <div className={style.content}>
        {selectedComponent}
      </div>
    </div>
  );
};

export default App;
