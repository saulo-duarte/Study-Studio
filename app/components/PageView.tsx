// reading/[id]/components/PageView.tsx
import { useEffect, useRef } from 'react';

export default function PageView({
  page,
  bookId,
}: {
  page: number;
  bookId: string;
}) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const loadPage = async () => {
      const pdf = await import('pdfjs-dist');
      const pdfDoc = await pdf.getDocument(`/books/${bookId}.pdf`).promise;
      const pdfPage = await pdfDoc.getPage(page);

      const viewport = pdfPage.getViewport({ scale: 1 });
      const canvas = canvasRef.current!;
      const context = canvas.getContext('2d')!;

      canvas.width = viewport.width;
      canvas.height = viewport.height;

      pdfPage.render({ canvasContext: context, viewport });
    };

    loadPage();
  }, [page, bookId]);

  return <canvas ref={canvasRef} />;
}
