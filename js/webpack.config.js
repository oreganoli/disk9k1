const path = require('path');
module.exports = {
    entry: './src/index.js',
    mode: "development",
    devtool: "source-map",
    module: {
        rules: [
            {
                test: /\.(js|jsx)$/,
                exclude: /node_modules/,
                use: {loader: 'babel-loader', options: {presets: ['@babel/preset-env', '@babel/preset-react']}}
            }
        ]
    },
    output: {
        path: __dirname + '/dist',
        publicPath: '/',
        filename: 'bundle.js'
    }
};