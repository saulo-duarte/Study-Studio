import * as React from "react";
import Image from 'next/image';
import { Card, CardContent } from "./ui/card";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "./ui/carousel";
import { Progress } from "./ui/progress";

type LibraryCarouselProps = {
  books: {
    title: string;
    imageUrl: string;
    pagesRead: number;
    totalPages: number;
  }[];
  type?: "continue-to-read";
};

export function LibraryCarousel({ books, type }: LibraryCarouselProps) {
  return (
    <Carousel
      opts={{
        align: "start",
      }}
      className="w-full max-w-full"
    >
      <CarouselContent>
        {books.map((book, index) => (
          <CarouselItem
            key={index}
            className={type === "continue-to-read" 
              ? "basis-full md:basis-[30%] lg:basis-[29%]"
              : "basis-full md:basis-1/6 lg:basis-1/6"
            }
          >
            <Card className="h-full">
              <CardContent className="flex flex-col h-full p-4">
                <div className="relative aspect-[2/3] w-full mb-4">
                  <Image
                    src={book.imageUrl}
                    alt={book.title}
                    className="rounded-lg object-cover"
                    fill={true}
                    sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
                    priority={index === 0}
                  />
                </div>
                <div className="flex flex-col gap-2">
                  <h3 className="text-sm font-medium line-clamp-2">{book.title}</h3>
                  <div className="text-xs text-muted-foreground">
                    {book.pagesRead} / {book.totalPages} pages read
                  </div>
                  <Progress value={(book.pagesRead / book.totalPages) * 100} className="h-1" />
                </div>
              </CardContent>
            </Card>
          </CarouselItem>
        ))}
      </CarouselContent>
      <CarouselPrevious />
      <CarouselNext />
    </Carousel>
  );
}
