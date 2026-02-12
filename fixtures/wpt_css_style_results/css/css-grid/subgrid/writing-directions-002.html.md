# css/css-grid/subgrid/writing-directions-002.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/writing-directions-002.html"
}
```

## style[0]

```css

html,body {
  font:12px/1 monospace;
}

.grid {
  display: grid;
  grid: 0.2em 1.4em / repeat(10, auto);
  border: 1px solid;
  padding: 0 0 0 0;
}

div > div {
  display: grid;
  grid-column: 1 / span 3;
  grid: auto / subgrid;
  border: 1px solid;
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

.rtl { direction:rtl; }
.ltr { direction:ltr; }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
