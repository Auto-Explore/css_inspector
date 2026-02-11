# css/css-break/flexbox/multi-line-row-flex-fragmentation-085.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-085.html"
}
```

## style[0]

```css

  .multicol {
    columns: 2;
    column-fill: auto;
    width: 100px;
    height: 100px;
    background: green;
  }
  .flex {
    display: flex;
    flex-wrap: wrap;
    background: red;
  }
  .flex>div {
    contain: size;
    width: 100%;
    height: 10px;
    background: green;
    break-before: column;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
