import "@/app/globals.css";


export default function LoginLayout({
    children,
  }: Readonly<{
    children: React.ReactNode;
  }>) {

    return (
        <html lang="en">
          <body
          >
            <main>
            {children}
            </main>
          </body>
        </html>
      );
    }