'use client'
import { useState } from "react";
import style from "@/styles/App.module.css";
import Header from '@/components/Header';
import Selector from "@/components/Selector";
import MissionsList from "@/components/MissionsList";
import AddMission from "@/components/AddMission";
import Status from "@/components/Status"
import { DailyMission } from "@/types/DailyMission";

type AppProps = {
  // ミッション追加のProps
  title: string,
  setTitle: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  description: null | string;
  setDescription: (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => void;
  submitHandle: () => Promise<void>;
  missions: DailyMission[];
  userStatus: null | Level;
}
const App = (props: AppProps) => {
  const {
    title,
    setTitle,
    description,
    setDescription,
    submitHandle,
    missions,
    userStatus,
  } = props;

  const [selectorVal, setSelectorVal] = useState(0);

  const handleSelectorChange = (_e: React.SyntheticEvent<Element, Event>, newValue: number) => {
    setSelectorVal(newValue);
  };

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
      selectedComponent = <Status userStatus={userStatus}/>;
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
