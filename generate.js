array = [[2],[1,1],[2,2],[1],[3,3],[2],[4,4],[1],[3],[4]]

let x = "";

function generate_put(arr) {
    return "c.put(" + arr[0] + "," + arr[1] + ")" + ";"
}

function generate_get(arr) {
    return "c.get(" + arr[0]+ ")" + ";"
}
for(let i of array){
    if (i.length == 1){
        x = x + generate_get(i)
    }
    else {
       x=x+generate_put(i)
    }
}

console.log(x);