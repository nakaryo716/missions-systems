'use client'
import { Button, Card, CardActions, CardContent, CardHeader } from "@mui/material";
import style from "../styles/Mission.module.css";
import { TaskAltOutlined } from "@mui/icons-material";

type MissionProps = {
  mission: DailyMission;
};

const Mission = (props: MissionProps) => {
  const {
    mission,
  } = props;

  return (
      <Card className={style.c}>
        <CardHeader title={mission.title}/>
        <CardContent>{mission.description}</CardContent>
        <div style={{display: "flex", justifyContent: "right"}}>
          <CardActions>
            <Button
              variant="contained"
              color="success"
              disabled={mission.isComplete}
              size="medium"
            >
              <div className={style.icon}>
                <TaskAltOutlined />
              </div>
              {mission.isComplete? "Done" : "Complete"}
            </Button>
          </CardActions>
        </div>
      </Card>
  );
};

export default Mission;
