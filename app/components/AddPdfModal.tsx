import { useState, useCallback, useEffect } from "react";
import { Modal, ModalContent, ModalHeader, ModalBody, ModalFooter } from "@nextui-org/react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { Button } from "./ui/button";
import { Progress } from "./ui/progress";
import { useToast } from "../hooks/use-toast";
import { X, BookOpen, User, Upload, Trash2 } from "lucide-react";
import { useDropzone } from "react-dropzone";
import { FaCheckCircle } from 'react-icons/fa';

interface AddPdfModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AddPdfModal({ isOpen, onClose }: AddPdfModalProps) {
  const [title, setTitle] = useState("");
  const [author, setAuthor] = useState("");
  const [file, setFile] = useState<File | null>(null);
  const [pagePreview, setPagePreview] = useState<string | null>(null);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [isUploading, setIsUploading] = useState(false);
  const { toast } = useToast();

  const [fieldErrors, setFieldErrors] = useState<{ title?: string; file?: string }>({});

  const generatePreview = async (file: File) => {
    try {
      
      const pdfjsLib = await import('pdfjs-dist');
      pdfjsLib.GlobalWorkerOptions.workerSrc = '/pdf.worker.min.mjs';

      const fileArrayBuffer = await file.arrayBuffer();
      const loadingTask = pdfjsLib.getDocument(fileArrayBuffer);
      const pdf = await loadingTask.promise;
      
      const page = await pdf.getPage(1);
      
      const viewport = page.getViewport({ scale: 1.0 });
      const canvas = document.createElement('canvas');
      const context = canvas.getContext('2d');
      
      if (!context) {
        throw new Error('Failed to get canvas context');
      }

      const scale = Math.min(400 / viewport.width, 300 / viewport.height);
      const scaledViewport = page.getViewport({ scale });

      canvas.width = scaledViewport.width;
      canvas.height = scaledViewport.height;

      // Renderizar a página no canvas
      const renderContext = {
        canvasContext: context,
        viewport: scaledViewport,
      };

      await page.render(renderContext).promise;
      
      // Converter o canvas para uma URL de dados
      const dataUrl = canvas.toDataURL('image/jpeg', 0.8);
      setPagePreview(dataUrl);

    } catch (error) {
      console.error('Error generating preview:', error);
      toast({
        variant: "destructive",
        title: "Error",
        description: "Failed to generate PDF preview.",
      });
    }
  };

  const handleCleanup = useCallback(() => {
    setTitle("");
    setAuthor("");
    setFile(null);
    setPagePreview(null);
    setUploadProgress(0);
    setIsUploading(false);
    setFieldErrors({});
  }, []);

  const handleClose = useCallback(() => {
    handleCleanup();
    onClose();
  }, [handleCleanup, onClose]);

  const onDrop = useCallback(async (acceptedFiles: File[]) => {
    if (acceptedFiles.length > 0) {
      const newFile = acceptedFiles[0];
      setFile(newFile);
      await generatePreview(newFile);
      setFieldErrors((prev) => ({ ...prev, file: undefined }));
    }
  }, []);

  const handleRemoveFile = useCallback((e: React.MouseEvent) => {
    e.stopPropagation();
    setFile(null);
    setPagePreview(null);
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: { "application/pdf": [] },
    multiple: false,
  });

  const simulateProgress = () => {
    setIsUploading(true);
    const interval = setInterval(() => {
      setUploadProgress((prev) => {
        if (prev >= 100) {
          clearInterval(interval);
          return 100;
        }
        return prev + 10;
      });
    }, 500);
    return interval;
  };

  const handleSubmit = async () => {
    const errors: { title?: string; file?: string } = {};
  
    if (!title) errors.title = "Title is required.";
    if (!file) errors.file = "PDF file is required.";
  
    setFieldErrors(errors);
  
    if (Object.keys(errors).length > 0) {
      toast({
        variant: "destructive",
        title: "Error",
        description: "Please fill all required fields.",
      });
      return;
    }
  
    try {
      const interval = simulateProgress();
  
      if (!file) throw new Error("File is required but not provided.");
  
      await invoke("insert_new_book_command", {
        title,
        author: author || null,
        filePath: file.name,
      });
  
      clearInterval(interval);
      setUploadProgress(100);

      toast({
        title: (
          <div className="flex items-center space-x-2">
            <FaCheckCircle className="text-white text-xl" />
            <span>Book Added!</span>
          </div>
        ) as unknown as string,
        description: "The book has been successfully added to your collection.",
        variant: "default",
        className: "bg-[#58c76a] text-black border-none shadow-xl rounded-lg flex items-center space-x-3 p-4 transform transition-all duration-500 ease-in-out hover:scale-105",
        duration: 5000,
        style: {
          fontSize: "16px",
          fontWeight: "600",
        },
      });
  
      setTimeout(handleClose, 1000);
    } catch (error) {
      toast({
        variant: "destructive",
        title: "Error",
        description: "Failed to add book. Please try again.",
      });
      setIsUploading(false);
      setUploadProgress(0);
    }
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      placement="center"
      hideCloseButton
      size="lg"
      backdrop="opaque"
      classNames={{
        backdrop: "bg-[#292f46]/50 backdrop-opacity-40",
        base: "bg-[#1f1f1f] text-white border-yellow-400",
        header: "border-b border-gray-700",
        body: "py-6 relative",
        footer: "border-t border-gray-700",
      }}
    >
      <ModalContent>
        <ModalHeader className="flex justify-between items-center py-4">
          <h2 className="text-xl font-bold text-center w-full">Add New Book</h2>
          <Button
            variant="ghost"
            size="icon"
            onClick={handleClose}
            className="absolute right-4 top-4 text-gray-400 hover:text-white"
          >
            <X className="h-4 w-4" />
          </Button>
        </ModalHeader>
        <ModalBody>
          <div className="space-y-6">
            <div className="space-y-2">
              <Label htmlFor="title" className="text-sm font-medium text-gray-300">
                Title
              </Label>
              <div className="relative">
                <BookOpen className="absolute left-2 top-2.5 h-4 w-4 text-gray-400" />
                <Input
                  id="title"
                  placeholder="Enter book title"
                  value={title}
                  onChange={(e) => {
                    setTitle(e.target.value);
                    setFieldErrors((prev) => ({ ...prev, title: undefined }));
                  }}
                  className={`pl-8 bg-background text-white border-gray-600 focus:border-yellow-400 focus:ring-yellow-400 ${
                    fieldErrors.title ? "border-red-500" : ""
                  }`}
                />
                {fieldErrors.title && <p className="text-sm text-red-500">{fieldErrors.title}</p>}
              </div>
            </div>
            <div className="space-y-2">
              <Label htmlFor="author" className="text-sm font-medium text-gray-300">
                Author
              </Label>
              <div className="relative">
                <User className="absolute left-2 top-2.5 h-4 w-4 text-gray-400" />
                <Input
                  id="author"
                  placeholder="Enter author name"
                  value={author}
                  onChange={(e) => setAuthor(e.target.value)}
                  className="pl-8 bg-background text-white border-gray-600 focus:border-yellow-400 focus:ring-yellow-400"
                />
              </div>
            </div>
            <div className="space-y-2">
              <Label className="text-sm font-medium text-gray-300">Upload PDF</Label>
              <div
                {...getRootProps()}
                className={`
                  border-2 border-dashed rounded-lg p-8 text-center cursor-pointer
                  transition-colors duration-200
                  ${
                    isDragActive
                      ? "border-yellow-400 bg-yellow-400/10"
                      : fieldErrors.file
                      ? "border-red-500"
                      : "border-gray-600 hover:border-yellow-400/50"
                  }
                `}
              >
                <input {...getInputProps()} />
                {file ? (
                  <div className="space-y-4">
                    <div className="relative w-full max-w-xs mx-auto">
                      {pagePreview ? (
                        <img
                          src={pagePreview}
                          alt="First page preview"
                          className="w-full h-48 object-contain "
                        />
                      ) : (
                        <div className="w-full h-48 flex items-center justify-center">
                          <p className="text-gray-400">Generating preview...</p>
                        </div>
                      )}
                      <Button
                        variant="destructive"
                        size="icon"
                        onClick={handleRemoveFile}
                        className="absolute -top-2 -right-2 h-6 w-6 rounded-full"
                      >
                        <Trash2 className="h-4 w-4" />
                      </Button>
                    </div>
                    <div className="space-y-1">
                      <p className="text-sm text-gray-300">{file.name}</p>
                      <p className="text-xs text-gray-400">
                        {(file.size / (1024 * 1024)).toFixed(2)} MB
                      </p>
                    </div>
                  </div>
                ) : (
                  <div className="space-y-2">
                    <Upload className="h-10 w-10 mx-auto mb-4 text-gray-400" />
                    <p className="text-sm text-gray-300">
                      {isDragActive ? "Drop your PDF here" : "Drag & drop your PDF here"}
                    </p>
                    <p className="text-xs text-gray-400">
                      or <span className="text-yellow-400">browse files</span>
                    </p>
                  </div>
                )}
                {fieldErrors.file && <p className="text-sm text-red-500 mt-2">{fieldErrors.file}</p>}
              </div>
            </div>
            {isUploading && (
              <div className="space-y-2">
                <div className="flex justify-between text-xs text-gray-400">
                  <span>Uploading...</span>
                  <span>{uploadProgress}%</span>
                </div>
                <Progress value={uploadProgress} className="h-1" />
              </div>
            )}
          </div>
        </ModalBody>
        <ModalFooter>
          <Button
            variant="outline"
            onClick={handleClose}
            className="mr-2 border-gray-600 text-gray-300 hover:bg-gray-700"
          >
            Cancel
          </Button>
          <Button
            onClick={handleSubmit}
            disabled={isUploading}
            className="bg-yellow-400 text-gray-900 hover:bg-yellow-500 disabled:opacity-50"
          >
            {isUploading ? "Uploading..." : "Add Book"}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  );
}