"use client";

import { usePathname } from 'next/navigation';
import "./globals.css";
import { SidebarProvider, SidebarTrigger } from "./components/ui/sidebar";
import { AppSidebar } from "./components/AppSidebar";
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useState } from 'react';
import { useRouter } from "next/navigation";
import { NextUIProvider } from '@nextui-org/system';

export default function RootLayout({
  children
}: {
  children: React.ReactNode
}) {
  const pathname = usePathname();
  const router = useRouter();
  const [isChecked, setIsChecked] = useState(false);

  useEffect(() => {
    const checkUserAndRedirect = async () => {
      try {
        const isActiveUser: boolean = await invoke('check_if_there_is_active_user_status_command');
        console.log("User active status:", isActiveUser);
        
        if (!isActiveUser) {
          console.log("Redirecting to onboarding...");
          router.push('/onboarding');
        }
        
        console.log("Setting isChecked to true");
        setIsChecked(true);
      } catch (e) {
        console.error("Error checking user status:", e);
        setIsChecked(true);
      }
    };

    checkUserAndRedirect();
  }, [router]);

  console.log("Current state - isChecked:", isChecked, "pathname:", pathname);

  return (
    <html lang="en" className="dark">
      <body>
        {isChecked && (
          <main>
            {!pathname?.includes('/onboarding') ? (
                  <NextUIProvider>

              <SidebarProvider>
                <AppSidebar />
                <SidebarTrigger />
                {children}
              </SidebarProvider>
                  </NextUIProvider>
            ) : (
              children
            )}
          </main>
        )}
      </body>
    </html>
  );
}
