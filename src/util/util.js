
export const find_exist = (item, list) => {
    if (item === null) return;
    if (list.length < 1) return;
    for (let i = 0; i < list.length; i++) {
        if (item.id === list[i].id) return list[i]
    }
    return null;
}

export const union_list = (data, list) => {
    for (let i = 0; i < data.length; i++) {
        let item = data[i]
        let exist = find_exist(item, list)
        if (exist != null) continue
        list.push(item)
    } 
    // return sort_list(list);
    return quickSort(list);
}

export const set_checked = (list, index) => {
    if (list === null || list === undefined || list.length < 1) return
    list.forEach(element => {
        element.checked = false
    });
    list[index].checked = true
}

export const sort_list = (list) => {
    list.sort((a, b) => {
        b.id - a.id > 0 
    })
}


export const quickSort = arr => {
    if (arr.length <= 1) {
      return arr;
    }
    
    const pivot = arr[arr.length - 1];
    const leftArr = [];
    const rightArr = [];
    
    for (let i = 0; i < arr.length - 1; i++) {
      if (arr[i].id > pivot.id) {
        leftArr.push(arr[i]);
      } else {
        rightArr.push(arr[i]);
      }
    }
    
    return [...quickSort(leftArr), pivot, ...quickSort(rightArr)];
}

export const formatRelativeTime = (timestamp) => {
  const now = new Date().getTime(); // 获取当前时间戳
  const difference = now - timestamp; // 计算与当前时间的差值（毫秒）

  // 定义时间单位及其对应的毫秒数
  const units = [
    { label: '年', value: 31536000000 }, // 年（大约，非闰年）
    { label: '个月', value: 2592000000 }, // 月（平均，假设每月30天）
    { label: '周', value: 604800000 }, // 周
    { label: '天', value: 86400000 }, // 天
    { label: '小时', value: 3600000 }, // 小时
    { label: '分钟', value: 60000 }, // 分钟
    { label: '秒', value: 1000 }, // 秒
  ];

  // 找出最接近的单位，并计算数量
  for (let unit of units) {
    if (difference >= unit.value) {
      const count = Math.floor(difference / unit.value);
      return `${count} ${unit.label}${count > 1 ? '' : ''}前`;
    }
  }

  // 差值小于1秒，返回“刚刚”
  return '刚刚';
}


export const stringToArrayBuffer = (string) => {
    const buffer = new ArrayBuffer(string.length);
    const bufferView = new Uint8Array(buffer);
    for (let i = 0; i < string.length; i++) {
        bufferView[i] = string.charCodeAt(i);
    }
    return buffer;
}
