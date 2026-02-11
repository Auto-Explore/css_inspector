# css/css-break/flexbox/single-line-column-flex-fragmentation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/single-line-column-flex-fragmentation-002.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 2;
    column-fill:auto;
    column-gap: 0px;
    height: 100px;
    width: 100px;
  }
  .flex {
    display: flex;
    flex-direction: column;
    height: 200px;
    width: 50px;
  }
  .flex > div {
    height: 10px;
    width: 50px;
  }
  .flex > div > div {
    background: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
