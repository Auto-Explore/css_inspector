# css/css-values/calc-rounding-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-rounding-001-ref.html"
}
```

## style[0]

```css

body {
  background: #f3f5f6;
}

div {
  font-size: 15px;
  --width: 401px;
  width: var(--width);
  margin: 20px;
  background: #fff;
  display: flex;
  flex-wrap: wrap;
}

b {
  height: 50px;
  background: red;
  width: calc((var(--width) - 4.5em) / 4); /* .5em gutters */
}

b:not(:last-child) {
  margin-right: 1.5em;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
