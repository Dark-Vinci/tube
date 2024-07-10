module.exports = function (api) {
  api.cache(true);
  return {
    presets: ['babel-preset-expo'],
    plugins: [
      [
        'module-resolver',
        {
          alias: {
            '@components': './src/Components',
            '@containers': './src/Containers',
            '@state': './src/State',
            '@navigation': './src/Navigations',
            '@': './src/*',
          },
        },
      ],
    ],
  };
};
