import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  distDir: "out",
  images: {
    domains: ["i.imgur.com"],
    unoptimized: true,
  },
  output: "export",
  webpack: (config) => {
    config.module.rules.push({
      test: /\.mjs$/,
      type: 'javascript/auto',
    });

    config.module.rules.push({
      test: /pdf\.worker\.min\.mjs$/,
      use: {
        loader: 'file-loader',
      },
    });

    if (!config.resolve.alias) {
      config.resolve.alias = {};
    }
    config.resolve.alias.canvas = false;

    return config;
  },
};

export default nextConfig;
