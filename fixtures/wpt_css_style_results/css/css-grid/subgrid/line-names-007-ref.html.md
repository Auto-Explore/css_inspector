# css/css-grid/subgrid/line-names-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-007-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:12px/1 monospace;
}

.grid {
  position: relative;
  display: grid;
  grid: 0.2em / repeat(10,30px);
  grid-auto-rows: 2em;
  border: 1px solid;
  padding: 1px 2px;
}

div > div {
  padding: 4px 0 1px 0;
  border: 3px solid black;
  background: grey;
  margin-left: 30px;
  width: 54px;
}

n {
  grid-row: 1;
  counter-increment: n;
}
n::before { content: counter(n, decimal); }

x {
  position: absolute;
  left:0; right:0;
  background: silver;
}

.hr { writing-mode: horizontal-tb; direction:rtl; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
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
