import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  distDir: "out",
  images: {
    domains: ["i.imgur.com"],
  },
  output: 'export',
};

export default nextConfig;
