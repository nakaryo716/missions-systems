import { List, ListItem } from "@mui/material";
import Mission from "./Mission";
import style from "../styles/MissionsList.module.css"
import { DailyMission } from "@/types/DailyMission";

type MissionsListProps = {
  missions: DailyMission[];
  handleComplete: (id: string) => Promise<void>;
  handleGetMission: () => Promise<void>;
}
const MissionsList = ({ missions, handleComplete, handleGetMission }: MissionsListProps) => {
  return (
    <List className={style.c}>
      {
        missions.filter(v => !v.isComplete).map(v => {
          return (
            <ListItem key={v.missionId}>
              <Mission mission={v} handleClick={handleComplete} handleGetMission={handleGetMission}/>
            </ListItem>
          );
        })
      }
      {
        missions.filter(v => v.isComplete).map(v => {
          return (
            <ListItem key={v.missionId}>
              <Mission mission={v} handleClick={handleComplete} handleGetMission={handleGetMission}/>
            </ListItem>
          );
        })
      }
    </List>
  );
};

export default MissionsList;
