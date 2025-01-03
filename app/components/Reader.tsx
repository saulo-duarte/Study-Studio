import { useState } from "react";

export default function Reader({ bookId }: {bookId: String}) {
    const [currentPage, setCurrentPage] = useState(1);

    const handlePageChanger = (direction: 'next' | 'prev') => {
        setCurrentPage((prev) =>(direction === 'next' ? prev + 1: prev - 1));
    };

    return (
        <div></div>
    )
}