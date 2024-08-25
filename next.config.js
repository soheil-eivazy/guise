/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  webpack(config) {
    config.experiments = { 
      asyncWebAssembly: true,
      layers: true
    }
    config.module.rules.push({
      test: /\.(glsl|vs|fs|vert|frag)$/i,
      use: ['raw-loader', 'glslify-loader'],
    });

    return config
  },
}

module.exports = nextConfig
