let path = require('path')
const ThemeColorReplacer = require('webpack-theme-color-replacer')
const { getThemeColors, modifyVars } = require('./src/utils/themeUtil')
const { resolveCss } = require('./src/utils/theme-color-replacer-extend')

function resolve(dir) {
  return path.join(__dirname, dir)
}

module.exports = {
  devServer: {
    port: 8000,
    proxy: {
      /** 系统管理 */
      '/system': {
        target: 'http://127.0.0.1:8080',
        ws: false,
        changeOrigin: true
      },
      /** 附件服务
       *  需另行开发
       */
      '/annex': {
        target: 'http://127.0.0.1:8080',
        ws: false,
        changeOrigin: true
      }
    }
  },
  pluginOptions: {
    'style-resources-loader': {
      preProcessor: 'less',
      patterns: [path.resolve(__dirname, "./src/theme/theme.less")],
    }
  },
  configureWebpack: config => {
    config.entry.app = ["babel-polyfill", "whatwg-fetch", "./src/main.js"];
    config.plugins.push(
      new ThemeColorReplacer({
        fileName: 'css/theme-colors-[contenthash:8].css',
        matchColors: getThemeColors(),
        injectCss: true,
        resolveCss
      })
    )
    devtool = 'inline-source-map'
  },
  chainWebpack: config => {
    config.resolve.alias
      .set('@$', resolve('src'))
      .set('@api', resolve('src/api'))
      .set('@services', resolve('src/services'))
      .set('@comp', resolve('src/components'))
      .set('@pages', resolve('src/pages'))
      .set('@layout', resolve('src/layout'))

    config
      .plugin('html')
      .tap(args => {
        args[0].title = 'YNOS'
        return args
      })

    const svgRule = config.module.rule('svg')
    svgRule.uses.clear()
    svgRule
      .oneOf('inline')
      .resourceQuery(/inline/)
      .use('vue-svg-icon-loader')
      .loader('vue-svg-icon-loader')
      .end()
      .end()
      .oneOf('external')
      .use('file-loader')
      .loader('file-loader')
      .options({
        name: 'assets/[name].[hash:8].[ext]'
      })

    // 生产环境下关闭css压缩的 colormin 项，因为此项优化与主题色替换功能冲突
    if (process.env.NODE_ENV === 'production') {
      config.plugin('optimize-css')
        .tap(args => {
          args[0].cssnanoOptions.preset[1].colormin = false
          return args
        })
    }
  },
  css: {
    loaderOptions: {
      less: {
        lessOptions: {
          modifyVars: modifyVars(),
          javascriptEnabled: true
        }
      }
    }
  },
  publicPath: process.env.NODE_ENV === 'production' ? '' : '',
  // publicPath: "/sys",
  outputDir: 'system',
  productionSourceMap: false,
  lintOnSave: false,
  transpileDependencies: []
}
