# css/css-grid/subgrid/line-names-006.html

```json
{
  "format_version": 3,
  "file": "css/css-grid/subgrid/line-names-006.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:12px/1 monospace; padding:0; margin:0;
}

div > div { background: grey; grid-column: 2 / span 2; }

i {
  grid-row: 1;
  counter-increment: i;
}
i::before { content: counter(i, decimal); }

x { background: silver; }

.hr { writing-mode: horizontal-tb; direction:rtl; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
