"use client";

import LibrarySearchBar from "../components/LibrarySearchBar";
import { Button } from "../components/ui/button";
import { GrDocumentUpdate } from "react-icons/gr";
import { LibraryCarousel } from "../components/LibrayCarousel";
import { RiBookShelfLine } from "react-icons/ri";
import { Badge } from "../components/ui/badge";
import { Progress } from "../components/ui/progress";
import { Card } from "../components/ui/card";
import { AddPdfModal } from "../components/AddPdfModal";
import { useState } from "react";
import { useToast } from "../hooks/use-toast";
import { Toaster } from "../components/ui/toaster";


export default function Library() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const openModal = () => setIsModalOpen(true);
  const closeModal = () => setIsModalOpen(false);
  const { toast } = useToast()

  return (
    <div className="relative h-screen w-full">
      <Toaster />
      <div className="sticky top-0 z-50 w-full backdrop-blur-md bg-background bg-opacity-50">
        <div className="flex items-center gap-4 px-6 py-3">
          <h1 className="text-3xl font-semibold">Library Studio</h1>
          <div className="flex-1">
            <LibrarySearchBar />
          </div>
          <Button>
            <RiBookShelfLine /> View All
          </Button>
          <Button onClick={openModal}>
            <GrDocumentUpdate /> Add PDF
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 px-6 my-8 w-full items-end">
        <Card className="p-6 space-y-6 rounded-lg shadow-md w-full">
          <div className="flex flex-col md:flex-row gap-6">
            <div className="flex-shrink-0 w-full md:w-[40%] aspect-[3/4] bg-muted rounded-lg h-[350px]" />
            <div className="flex flex-col space-y-4 w-full">
              <h2 className="text-2xl font-bold break-words max-w-full">
                Designing Data Intensive Applications
              </h2>
              <div className="flex gap-2 flex-wrap">
                <Badge variant="secondary">Data Engineering</Badge>
                <Badge variant="secondary">System Designing</Badge>
              </div>
              <div className="space-y-2">
                <div className="text-sm text-muted-foreground">204/567 pages read</div>
                <Progress value={36} className="h-2" />
              </div>
            </div>
          </div>
        </Card>

        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-xl font-semibold">Continue to Read</h3>
          </div>
          <LibraryCarousel
            books={[
              {
                title: "Designing Data Intensive Applications",
                imageUrl: "https://m.media-amazon.com/images/I/81+oMD7Lm7L._UF894,1000_QL80_.jpg",
                pagesRead: 204,
                totalPages: 567,
              },
              {
                title: "Learning SQL",
                imageUrl: "https://cdn.analyticsvidhya.com/wp-content/uploads/2020/02/91pzJipjnL.jpg",
                pagesRead: 150,
                totalPages: 300,
              },
            ]}
            type="continue-to-read"
          />
        </div>
      </div>

      <div className="px-6 my-8">
        <LibraryCarousel
          books={[
            { title: "Designing Data Intensive Applications", imageUrl: "https://m.media-amazon.com/images/I/81+oMD7Lm7L._UF894,1000_QL80_.jpg", pagesRead: 204, totalPages: 567 },
            { title: "Learning SQL", imageUrl: "https://cdn.analyticsvidhya.com/wp-content/uploads/2020/02/91pzJipjnL.jpg", pagesRead: 150, totalPages: 300 },
          ]}
        />
      </div>
      <AddPdfModal isOpen={isModalOpen} onClose={closeModal} />
    </div>
  );
}