const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist/jump"),
    filename: "index.js",
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: "Error"
    })
  ],
  // mode: "development"
  mode: "production"
};