import enquireJs from 'enquire.js'

export function timeFix() {
  const time = new Date()
  const hour = time.getHours()
  return hour < 9 ? '早上好' : hour <= 11 ? '上午好' : hour <= 13 ? '中午好' : hour < 20 ? '下午好' : '晚上好'
}

export function welcome() {
  const arr = ['休息一会儿吧', '准备吃什么呢?', '要不要打一把 DOTA', '我猜你可能累了']
  const index = Math.floor(Math.random() * arr.length)
  return arr[index]
}

export function isDef(v) {
  return v !== undefined && v !== null
}

/**
 * Remove an item from an array.
 */
export function remove(arr, item) {
  if (arr.length) {
    const index = arr.indexOf(item)
    if (index > -1) {
      return arr.splice(index, 1)
    }
  }
}

export function isRegExp(v) {
  return _toString.call(v) === '[object RegExp]'
}

export function enquireScreen(call) {
  const handler = {
    match: function () {
      call && call(true)
    },
    unmatch: function () {
      call && call(false)
    }
  }
  enquireJs.register('only screen and (max-width: 767.99px)', handler)
}

const _toString = Object.prototype.toString


/**
 * @description: 密码强度校验
 * @param {String}  sValue 需要校验的密码
 * @return {Number} modes 密码强度数值 1.弱 2.中 3.强 
 * @author: jiajun wu
 */
export function checkStrong(sValue) {
  var modes = 0;
  //正则表达式验证符合要求的
  if (sValue.length < 1) return modes;
  if (/\d/.test(sValue)) modes++; //数字
  if (sValue.length >= 6 && /[a-z]/.test(sValue)) modes++; //小写
  if (sValue.length >= 6 && /[A-Z]/.test(sValue)) modes++; //大写
  if (sValue.length >= 6 && /\W/.test(sValue)) modes++; //特殊字符

  //逻辑处理
  switch (modes) {
    case 1:
      return 1;
    case 2:
      return 2;
    case 3:
    case 4:
      return sValue.length < 4 ? 3 : 4;
  }
  return modes;
}