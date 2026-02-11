# css/css-grid/grid-container-baseline-synthesized-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/grid-container-baseline-synthesized-002.html"
}
```

## style[0]

```css

:root {
  font: 20px/1 Ahem;
}
.container {
  display: inline-grid;
  grid-template-rows: 60px;
  grid-template-columns: 60px;
  background: cyan;
  writing-mode: vertical-rl;
}
.f { display: flex; }
.g { display: grid; }
.t { display: table; }
.container > * {
  border: 3px solid black;
}
.vrl-items > * { writing-mode: vertical-rl; }
.vlr-items > * { writing-mode: vertical-lr; }
.srl-items > * { writing-mode: sideways-rl; }
.slr-items > * { writing-mode: sideways-lr; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
