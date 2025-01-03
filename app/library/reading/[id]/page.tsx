import { useRouter } from "next/router";
import { useState } from "react";

export default function ReadingPage(){
    const router = useRouter()
    const { id } = router.query;

    if(!id) return <div>Loading...</div>

    return (
        <div>
            
        </div>
    );
};