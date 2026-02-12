# css/css-grid/alignment/grid-content-alignment-with-span-vertical-rl-001.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/alignment/grid-content-alignment-with-span-vertical-rl-001.html"
}
```

## style[0]

```css

body {
  margin: 0px;
}

.grid {
  grid-auto-columns: 20px;
  grid-auto-rows: 40px;
  grid-template-areas: "a a b"
                       "c d b";
  position: relative;
  width: 200px;
  height: 300px;
}
.a {
  grid-area: a;
  background-color: blue;
}
.b {
  grid-area: b;
  background-color: lime;
}
.c {
  grid-area: c;
  background-color: purple;
}
.d {
  grid-area: d;
  background-color: orange;
}
.stretchedGrid {
  grid-auto-columns: auto;
  grid-auto-rows: auto;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
