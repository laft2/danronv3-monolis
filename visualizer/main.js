const removeAllChildren = (parent) => {
  while (parent.firstChild) {
    parent.removeChild(parent.firstChild);
  }
}
const findColor = (s) => {
  let val = [[210, 210, 211], [209, 165, 251], [116, 195, 231], [206, 189, 99]]
  if (s == "1") return `rgb(${val[0][2]}, ${val[0][1]}, ${val[0][0]})`;
  if (s == "2") return `rgb(${val[1][2]}, ${val[1][1]}, ${val[1][0]})`;
  if (s == "3") return `rgb(${val[2][2]}, ${val[2][1]}, ${val[2][0]})`;
  if (s == "4") return `rgb(${val[3][2]}, ${val[3][1]}, ${val[3][0]})`;
  return "rgb(255, 255, 255)"
}
let idx = 0;
let t;
let field;

const DX = [1, 0, -1, 0];
const DY = [0, 1, 0, -1];
let hist = [];
const init = () => {
  idx = 0;
  let map = document.getElementById("map").value;
  map = map.split("\n").map((v) => v.split(",").map(x => parseInt(x)));
  field = structuredClone(map);
  t = document.getElementById("t").value
  t = t.split("\n");
  t = t.map(v => {
    v = v.replaceAll(/[()]/g, "");
    v.replaceAll()
    v = v.trim();
    return v.split(",").map(x => parseInt(x));
  })
  hist = [];
}

const count = () => {
  let cnt = 0;
  field.forEach(rows => {
    rows.forEach(elm => {
      cnt += elm == 0;
    })
  })
  return cnt;
}

const update = () => {
  let tablet = document.createElement("table");
  field.forEach((rows, rid) => {
    let trt = document.createElement("tr");
    rows.forEach((elm, cid) => {
      let tdt = document.createElement("td");
      tdt.innerText = elm;
      if (t !== undefined && idx !== t.length && t[idx][0] == rid && t[idx][1] == cid) {
        tdt.style.border = "4px solid red";
      } else {
        tdt.style.border = "1px solid black"
      }
      tdt.style.width = "2ex"
      tdt.style.height = "2ex"
      tdt.style.textAlign = "center"
      tdt.style.background = findColor(elm)
      trt.appendChild(tdt);
    });
    tablet.appendChild(trt);
  });
  removeAllChildren(vis);
  vis.appendChild(tablet);

  if (idx == t.length) {
    document.getElementById("next").setAttribute("disabled", true)
  } else {
    document.getElementById("next").removeAttribute("disabled")
  }

  document.getElementById("count").innerText = count();

}

const next_field = iv => {
  let target = [];
  let used = Array.from(new Array(11), _ => new Array(22).fill(0));
  let que = [];
  let color = field[iv[0]][iv[1]];
  if (color == 0) {
    return;
  }
  que.push(iv);
  while (que.length !== 0) {
    let v = que.shift();
    if (used[v[0]][v[1]] == 1) {
      continue;
    }
    used[v[0]][v[1]] = 1;
    if (field[v[0]][v[1]] != color) {
      continue;
    }
    target.push(v);
    for (let i of [0, 1, 2, 3]) {
      let ny = v[0] + DY[i];
      let nx = v[1] + DX[i];
      if (ny < 0 || 11 <= ny || nx < 0 || 22 <= nx) {
        continue;
      }
      if (used[ny][nx] != 0) {
        continue;
      }

      used[ny][nx] = -1;
      que.push([ny, nx])
    }
  }
  if (target.length <= 1) {
    return false;
  }
  used = Array.from(new Array(11), _ => new Array(22).fill(0));
  for (let ele of target) {
    field[ele[0]][ele[1]] = 0;
    used[ele[0]][ele[1]] = 1;
    for (let i of [0, 1, 2, 3]) {
      let ny = ele[0] + DY[i];
      let nx = ele[1] + DX[i];
      if (ny < 0 || 11 <= ny || nx < 0 || 22 <= nx) {
        continue;
      }
      if (used[ny][nx] == 1) {
        continue;
      }
      used[ny][nx] = 1;
      field[ny][nx] += 1;
      if (field[ny][nx] == 5) {
        field[ny][nx] = 1
      } else if (field[ny][nx] == 1) {
        field[ny][nx] = 0
      }
    }
  }
}

document.getElementById("start").addEventListener("click", () => {
  init();
  update();
});

document.getElementById("next").addEventListener("click", () => {
  hist.push([idx, structuredClone(t), structuredClone(field)]);
  next_field(t[idx]);
  idx++;
  update();
});

document.getElementById("prev").addEventListener("click", () => {
  [idx, t, field] = hist.pop()
  update();
})