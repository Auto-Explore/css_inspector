# css/css-grid/subgrid/line-names-012-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-012-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:12px/1 monospace;
}

.grid {
  display: grid;
  grid: 0.2em 1.4em / repeat(10,30px);
  border: 1px solid;
  padding: 0 2px;
}

div > div {
  xposition: relative;
  display: grid;
  grid: auto / subgrid;
  border: 1px solid black;
  background: grey;
}

n {
  grid-row: 1;
  counter-increment: n;
}
n::before { content: counter(n, decimal); }

x {
  background: silver;
}

.hr { writing-mode: horizontal-tb; direction:rtl; }
  
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “grid”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “xposition”.",
      "severity": "Error"
    },
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
