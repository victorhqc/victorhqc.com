const {
  PHASE_DEVELOPMENT_SERVER,
  PHASE_PRODUCTION_BUILD,
} = require('next/constants');

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['unsplash.com'],
  },
};

module.exports = (phase) => {
  const isDev = phase === PHASE_DEVELOPMENT_SERVER;
  const isProd = phase === PHASE_PRODUCTION_BUILD;

  return {
    env: {
      API_URL: isDev ? 'http://127.0.0.1:7878' : 'https://api.victorhqc.com',
    },
    webpack: (config, _options) => {
      config.module.rules.push({
        test: /\.node$/,
        loader: 'node-loader',
      });

      // Important: return the modified config
      return config;
    },
    ...nextConfig,
  };
};
