import { Input } from "@nextui-org/react";

export default function LibrarySearchBar() {

  return (
        <div className="w-full">
          <Input 
            placeholder="Search your books here"
            color={"default"} 
            type="text" 
            variant={"flat"} 
          />
        </div>
  );
}
