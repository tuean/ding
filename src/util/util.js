
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
    return sort_list(list);
}

export const sort_list = (list) => {
    list.sort((a, b) => {
        a.data - b.data
    })
    return sort_list
}