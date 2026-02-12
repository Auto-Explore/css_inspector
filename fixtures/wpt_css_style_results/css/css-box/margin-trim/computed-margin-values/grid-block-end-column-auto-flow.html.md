# css/css-box/margin-trim/computed-margin-values/grid-block-end-column-auto-flow.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/grid-block-end-column-auto-flow.html"
}
```

## style[0]

```css

grid {
    display: grid;
    width: min-content;
    grid-auto-flow: column;
    border: 1px solid black;
    grid-template-rows: auto auto auto;
    margin-trim: block-end;
}
item {
    display: block;
    width: 50px;
    height: 50px;
    margin-block-end: 10px;
}
.span-four {
    grid-row: span 4;
}
item:nth-child(1) {
    grid-row: 1;
    background-color: green;
}
item:nth-child(2) {
    grid-row: 2;
    background-color: blue;
}
item:nth-child(3) {
    grid-row: 3;
    background-color: purple;
}
item:nth-child(4) {
    background-color:burlywood;
}
item:nth-child(5) {
    background-color: grey;
    grid-row: 4;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
