const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = () => {
  return {
    mode: 'development',
    entry: {
      main: path.join(__dirname, 'src/index.ts'),
      sw: path.join(__dirname, 'src/sw.ts'),
    },
    target: ['webworker'],
    output: {
      path: path.join(__dirname, 'dist'),
      filename: '[name].js',
      publicPath: '/',
    },
    module: {
      rules: [
        {
          test: /\.tsx?$/,
          exclude: /node_modules/,
          use: [
            {
              loader: 'ts-loader',
            },
          ],
        },
      ],
    },
    plugins: [
      new HtmlWebpackPlugin({
        template: 'src/index.html',
        filename: 'index.html',
        chunks: ['main'],
      }),
    ],
  };
};
