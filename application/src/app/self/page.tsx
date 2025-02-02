'use client'
import getUserInfoApi from "@/api/userInfoApi";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function MyPage() {
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
  
  return(
    <>
      {
        loading ? 
          <div style={{textAlign: "center", alignItems: "center"}}>
            <h1>Now Loading...</h1>
          </div>
        : <h1>My Page</h1>
      }
    </>
  );
}
