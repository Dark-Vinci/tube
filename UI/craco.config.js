var path = require('path');

module.exports = {
    webpack: {
        alias: {
            '@': path.resolve(__dirname, 'src'),
            '@components': path.resolve(__dirname, 'src/Components'),
            '@pages': path.resolve(__dirname, 'src/Pages'),
            '@containers': path.resolve(__dirname, 'src/Containers'),
            '@store': path.resolve(__dirname, 'src/Store'),
            '@helper': path.resolve(__dirname, 'src/Helper'),
            '@router': path.resolve(__dirname, 'src/Router'),
            '@startup': path.resolve(__dirname, 'src/Startup'),
        },
    },
};
