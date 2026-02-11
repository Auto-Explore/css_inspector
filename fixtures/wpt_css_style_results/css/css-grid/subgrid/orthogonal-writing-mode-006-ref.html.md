# css/css-grid/subgrid/orthogonal-writing-mode-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/orthogonal-writing-mode-006-ref.html"
}
```

## style[0]

```css

body {
  background: white;
  color: black;
  display: grid;
  font: 20px/1 Ahem;
  place-items: start;
  padding: 0;
  margin: 0;
}
.grid {
  background: cyan;
  display: grid;
  padding: 5px;
}
.grid > .grid {
  background: pink;
}
.h { writing-mode: horizontal-tb; }
.v { writing-mode: vertical-rl; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
