# css/css-break/flexbox/multi-line-row-flex-fragmentation-067.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-067.html"
}
```

## style[0]

```css

  .multicol {
    columns: 2;
    column-fill: auto;
    width: 300px;
    height: 100px;
    margin: 20px;
    background: yellow;
  }
  .flex {
    display: flex;
    flex-wrap: wrap;
    background: gray;
  }
  .flex > div {
    contain: size;
    width: 100%;
    height: 50px;
    background: cyan;
    break-before: always;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
