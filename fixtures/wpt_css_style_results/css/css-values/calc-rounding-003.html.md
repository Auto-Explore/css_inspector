# css/css-values/calc-rounding-003.html

```json
{
  "format_version": 3,
  "file": "css/css-values/calc-rounding-003.html"
}
```

## style[0]

```css

  .outer {
    width: 100px;
    border: 1px solid;
  }
  .inner {
    height: 40px;
    vertical-align: top;
    display: inline-block;
    --margin: 4.009px;
    width: calc(50% - 2 * var(--margin));
    margin-inline: var(--margin);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
