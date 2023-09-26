
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

export const set_checked = list => {
    if (list === null || list === undefined || list.length < 1) return
    list.forEach(element => {
        element.checked = false
    });
    list[0].checked = true
}

export const sort_list = (list) => {
    [...list].sort((a, b) => {
        b.id - a.id > 0 
    })
    return list
}