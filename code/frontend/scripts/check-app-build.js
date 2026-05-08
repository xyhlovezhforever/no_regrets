/**
 * App打包前检查脚本
 * 检查必要的配置和文件是否完整
 */

const fs = require('fs');
const path = require('path');

// 颜色输出
const colors = {
  reset: '\x1b[0m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function checkManifest() {
  log('\n📋 检查 manifest.json 配置...', 'blue');
  
  const manifestPath = path.join(__dirname, '../src/manifest.json');
  
  if (!fs.existsSync(manifestPath)) {
    log('❌ manifest.json 文件不存在', 'red');
    return false;
  }
  
  try {
    const manifestContent = fs.readFileSync(manifestPath, 'utf-8');
    // 移除注释
    const cleanContent = manifestContent.replace(/\/\*[\s\S]*?\*\/|\/\/.*/g, '');
    const manifest = JSON.parse(cleanContent);
    
    let hasError = false;
    
    // 检查基本信息
    if (!manifest.name || manifest.name === '') {
      log('❌ 应用名称(name)未配置', 'red');
      hasError = true;
    } else {
      log(`✅ 应用名称: ${manifest.name}`, 'green');
    }
    
    if (!manifest.appid || manifest.appid === '' || manifest.appid === '__UNI__XXXXXX') {
      log('⚠️  AppID未配置或使用默认值，需要在HBuilderX中重新获取', 'yellow');
      log('   提示: 在HBuilderX中打开项目，会自动生成AppID', 'yellow');
    } else {
      log(`✅ AppID: ${manifest.appid}`, 'green');
    }
    
    if (!manifest.description || manifest.description === '') {
      log('⚠️  应用描述(description)未配置', 'yellow');
    } else {
      log(`✅ 应用描述: ${manifest.description}`, 'green');
    }
    
    if (!manifest.versionName) {
      log('❌ 版本名称(versionName)未配置', 'red');
      hasError = true;
    } else {
      log(`✅ 版本名称: ${manifest.versionName}`, 'green');
    }
    
    if (!manifest.versionCode) {
      log('❌ 版本号(versionCode)未配置', 'red');
      hasError = true;
    } else {
      log(`✅ 版本号: ${manifest.versionCode}`, 'green');
    }
    
    // 检查App配置
    if (!manifest['app-plus']) {
      log('❌ app-plus 配置缺失', 'red');
      hasError = true;
    } else {
      log('✅ app-plus 配置存在', 'green');
    }
    
    return !hasError;
  } catch (error) {
    log(`❌ manifest.json 解析失败: ${error.message}`, 'red');
    return false;
  }
}

function checkDependencies() {
  log('\n📦 检查依赖安装...', 'blue');
  
  const nodeModulesPath = path.join(__dirname, '../node_modules');
  
  if (!fs.existsSync(nodeModulesPath)) {
    log('❌ node_modules 不存在，请先运行 npm install', 'red');
    return false;
  }
  
  log('✅ 依赖已安装', 'green');
  return true;
}

function checkAppPlusModule() {
  log('\n🔌 检查 App 模块...', 'blue');
  
  const appPlusPath = path.join(__dirname, '../node_modules/@dcloudio/uni-app-plus');
  
  if (!fs.existsSync(appPlusPath)) {
    log('❌ @dcloudio/uni-app-plus 模块不存在', 'red');
    log('   请运行: npm install @dcloudio/uni-app-plus', 'yellow');
    return false;
  }
  
  log('✅ App 模块已安装', 'green');
  return true;
}

function printBuildInstructions() {
  log('\n📱 App打包说明:', 'blue');
  log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'blue');
  log('\n方式一: 使用 HBuilderX (推荐)', 'yellow');
  log('  1. 下载并安装 HBuilderX App开发版');
  log('  2. 导入本项目到 HBuilderX');
  log('  3. 点击 发行 -> 原生App-云打包');
  log('  4. 选择 Android 或 iOS');
  log('  5. 配置证书（测试可用公共证书）');
  log('  6. 等待打包完成并下载');
  
  log('\n方式二: 命令行构建资源', 'yellow');
  log('  npm run build:app          # 构建App资源');
  log('  npm run build:app-android  # 构建Android资源');
  log('  npm run build:app-ios      # 构建iOS资源');
  log('  注意: 命令行只生成资源，最终打包仍需HBuilderX或原生环境');
  
  log('\n详细文档请查看: 打包App指南.md', 'green');
  log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n', 'blue');
}

function main() {
  log('🚀 开始检查App打包环境...', 'blue');
  log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'blue');
  
  const checks = [
    checkManifest(),
    checkDependencies(),
    checkAppPlusModule(),
  ];
  
  const allPassed = checks.every(result => result);
  
  log('\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━', 'blue');
  
  if (allPassed) {
    log('✅ 所有检查通过！可以开始打包', 'green');
    printBuildInstructions();
    process.exit(0);
  } else {
    log('❌ 检查未通过，请修复上述问题后再打包', 'red');
    process.exit(1);
  }
}

main();
