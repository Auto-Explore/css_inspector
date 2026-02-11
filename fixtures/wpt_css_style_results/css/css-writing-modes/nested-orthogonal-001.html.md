# css/css-writing-modes/nested-orthogonal-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/nested-orthogonal-001.html"
}
```

## style[0]

```css

body > div {
  background: green;
  writing-mode: vertical-lr;
}

div > div { writing-mode: horizontal-tb; }

div > div > div {
  writing-mode: vertical-rl;
  height: 100px;
  font-size: 100px;
  line-height: 1;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
