"use client";
import {
  Button,
  Card,
  CardActions,
  CardContent,
  CardHeader,
} from "@mui/material";
import style from "../styles/Mission.module.css";
import { TaskAltOutlined } from "@mui/icons-material";
import { DailyMission } from "@/types/DailyMission";
import EditIcon from "@mui/icons-material/Edit";
import { useState } from "react";
import EditModal from "./EditModal";
import deleteMissionApi from "@/api/deleteMissionApi";
import updateMissionApi from "@/api/updateMissionApi";

type MissionProps = {
  mission: DailyMission;
  handleClick: (id: string) => Promise<void>;
  handleGetMission: () => Promise<void>;
};

const Mission = (props: MissionProps) => {
  const { mission, handleClick, handleGetMission } = props;
  const [open, setOpen] = useState(false);
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState<null | string>(null);
  // モーダルウィンドウを開くためのハンドラ
  const handleOpen = () => {
    setTitle(mission.title);
    setDescription(mission.description);
    setOpen(true);
    return;
  };
  // モーダルウィンドウを閉じるためのハンドラ
  const handleClose = () => {
    setOpen(false);
    setTitle("");
    setDescription(null);
    return;
  };

  const handleSetTitle = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setTitle(e.target.value);
    return;
  };

  const handleSetDescription = (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
  ) => {
    setDescription(e.target.value);
    return;
  };

  const handleEditMission = async (title: string, description: string | null) => {
    const updateMission: DailyMissionInput = {
      title,
      description,
    };

    const res = await updateMissionApi(mission.missionId, updateMission);
    if (!res.ok) {
      alert("編集に失敗しました");
      handleClose();
      return;
    }
    handleGetMission();
    handleClose();
    return;
  };

  const handleDeleteMission = async () => {
    const res = await deleteMissionApi(mission.missionId);
    if (!res.ok) {
      alert("削除に失敗しました");
      handleClose();
      return;
    }
    handleGetMission();
    handleClose();
    return;
  };

  return (
    <>
      <Card className={style.c}>
        <CardHeader title={mission.title} />
        <CardContent>{mission.description}</CardContent>
        <div style={{ display: "flex", justifyContent: "right" }}>
          <CardActions>
            <Button
              variant="contained"
              color="warning"
              size="medium"
              onClick={handleOpen}
            >
              <div className={style.icon}>
                <EditIcon />
              </div>
              Edit
            </Button>
            <Button
              variant="contained"
              color="success"
              disabled={mission.isComplete}
              size="medium"
              onClick={() => handleClick(mission.missionId)}
            >
              <div className={style.icon}>
                <TaskAltOutlined />
              </div>
              {mission.isComplete ? "Done" : "Complete"}
            </Button>
          </CardActions>
        </div>
      </Card>
      {/* モーダルウィンドウのコンポーネント */}
      <EditModal
        open={open}
        handleClose={handleClose}
        title={title}
        handleSetTitle={handleSetTitle}
        description={description}
        handleSetDescription={handleSetDescription}
        editMissionHandle={handleEditMission}
        deleteMissionHandle={handleDeleteMission}
      />
    </>
  );
};

export default Mission;
