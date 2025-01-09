const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html']),
  ],
  experiments: {
    asyncWebAssembly: true
  },
  module: {
    rules: [{
      test: /\.m?js/,
      resolve: {
        fullySpecified: false
      }
    }
    ],
  },
};
