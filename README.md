# swc-plugin-miniprogram-require-async

## 背景

[小程序通过分包异步化的能力](https://developers.weixin.qq.com/miniprogram/dev/framework/subpackages/async.html)，实现了跨分包 JS 代码/组件引用，解决了主包体积限制问题。但是，在标准化/工程化的今天，其提供的 `require.async` 语法**不符合标准**/**缺失编码提示**

该插件基于 [swc](https://swc.rs/docs/configuration/compilation)，将代码中 `import(path)` 转换成 `require.async`

## 使用方法

```javascript
// swc config

{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-miniprogram-require-async", {}]
      ]
    }
  }
}
```

## 转换结果

```javascript
// input
async function enter(roomId) {
  const enterVoiceRoom = await import('@voiceRoom/serviceUtils/enterRoom');
  enterVoiceRoom(roomId);
}

// output
async function enter(roomId) {
  const enterVoiceRoom = await require.async('@voiceRoom/serviceUtils/enterRoom');
  enterVoiceRoom(roomId);
}
```
