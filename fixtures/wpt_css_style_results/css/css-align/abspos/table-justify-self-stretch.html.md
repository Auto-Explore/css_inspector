# css/css-align/abspos/table-justify-self-stretch.html

```json
{
  "format_version": 3,
  "file": "css/css-align/abspos/table-justify-self-stretch.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

.container {
  writing-mode: horizontal-tb;
  display: inline-block;
  position: relative;
  margin: 20px;
  border: solid 4px;
  width: 40px;
  height: 40px;
}

.item {
  writing-mode: horizontal-tb;
  position: absolute;
  background: green;
  inset: 5px;
  justify-self: stretch;
  display: table;
}

.item::before {
  content: '';
  display: block;
  width: 10px;
  height: 20px;
}

.big::before {
  width: 50px;
  height: 60px;
}

.vrl {
  writing-mode: vertical-rl;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
