"use client"
import { usePathname } from 'next/navigation';
import "./globals.css";
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";


export default function RootLayout(
  { children }: { 
    children: React.ReactNode 
  }) {
  const pathname = usePathname();

  const showSidebar = !pathname?.includes('/onboarding');

  return (
    <html lang="en">
      <body>
        <main>
          {showSidebar && (
            <SidebarProvider>
              <AppSidebar />
              <SidebarTrigger />
              {children}
            </SidebarProvider>
          )}
        </main>
      </body>
    </html>
  );
}