//target area: x=20..30, y=-10..-5
//let [minX, maxX] = [20, 30]
//let [minY, maxY] = [-10, -5]
// target area: x=79..137, y=-176..-117
let [minX, maxX] = [79, 137]
let [minY, maxY] = [-176, -117]


window.findY = function() {
    let vy = minY;
    let cnt = 0;
    let tries = 0;
    let ys = [];

    while(tries++ < 10000) {
        if (testYForHit(vy)) {
            ys.push(vy);
        }
        vy++;
    }
    //console.log(ys)

    tries = 0;
    let vx = 0;
    let xs = [];
    while(tries++ < 10000) {
        let x = 0;
        let cvx = vx;
        while (cvx > 0) {
            x += cvx;
            if (x <= maxX) {
                if (x >= minX) {
                    //console.log("? " , ivy, my);
                    xs.push(vx);
                    break;
                }
            }
            cvx--;
        }
        vx++;
    }
    //console.log(xs)

    console.log(xs)
    console.log(ys)
    window.candidates = xs.flatMap(x => ys.map(y => {return {x, y}}));
    console.log(candidates)
    console.log(candidates.filter(p => testForHit(p.x, p.y)));
}

function testYForHit(ivy) {
    let y = 0
    let vy = ivy;
    let my = -Infinity;
    while (1) {
        y += vy;
        //console.log("  " + y)
        my = Math.max(my, y);
        if (y <= maxY) {
            if (y >= minY) {
                //console.log("? " , ivy, my);
                return true;
            } else {
                return false;
            }
        }
        vy--;
    }
}

window.testForHit = function(ivx, ivy) {
    let x = 0
    let y = 0
    let vx = ivx;
    let vy = ivy;
    while (1) {
        x += vx;
        y += vy;
        if (y >= minY) {
            if (x >= minX && x <= maxX && y <= maxY) {
                return true;
            }
        }
        if (y < minY) return false;
        vy--;
        vx = Math.max(0, --vx);
    }
}
