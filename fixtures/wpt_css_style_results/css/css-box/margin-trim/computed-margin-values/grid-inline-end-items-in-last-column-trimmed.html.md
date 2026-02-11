# css/css-box/margin-trim/computed-margin-values/grid-inline-end-items-in-last-column-trimmed.html

```json
{
  "format_version": 3,
  "file": "css/css-box/margin-trim/computed-margin-values/grid-inline-end-items-in-last-column-trimmed.html"
}
```

## style[0]

```css

grid {
    display: grid;
    grid-template-columns: repeat(2, auto);
    outline: 1px solid black;
    margin-trim: inline-end;
}
item {
    display: block;
    height: 50px;
}
.locked-position {
    grid-row: 2;
    grid-column: 2;
}
item:nth-child(1) {
    background-color: aqua;
    margin-inline-end: 10px;
}
item:nth-child(2) {
    background-color: blueviolet;
    margin-inline-end: 30%;
}
item:nth-child(3) {
    background-color: blue;
    margin-inline-end: -30px;
}
item:nth-child(4) {
    background-color: coral;
    margin-inline-end: 10px;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “margin-trim”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
