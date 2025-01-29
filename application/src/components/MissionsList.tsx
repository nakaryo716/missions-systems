import { List, ListItem } from "@mui/material";
import Mission from "./Mission";
import style from "../styles/MissionsList.module.css"

type MissionsListProps = {
  missions: DailyMission[];
}
const MissionsList = ({ missions }: MissionsListProps) => {
  return (
    <List className={style.c}>
      {
        missions.filter(v => !v.isComplete).map(v => {
          return (
            <ListItem key={v.missionId}>
              <Mission mission={v}/>
            </ListItem>
          );
        })
      }
      {
        missions.filter(v => v.isComplete).map(v => {
          return (
            <ListItem key={v.missionId}>
              <Mission mission={v} />
            </ListItem>
          );
        })
      }
    </List>
  );
};

export default MissionsList;
