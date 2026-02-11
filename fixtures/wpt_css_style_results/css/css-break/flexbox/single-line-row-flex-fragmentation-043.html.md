# css/css-break/flexbox/single-line-row-flex-fragmentation-043.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/single-line-row-flex-fragmentation-043.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 4;
    column-gap: 0;
    column-fill: auto;
    background: red;
    width: 100px;
    height: 100px;
  }

  .multicol > div {
    background: green;
  }

  .flexbox {
    display: flex;
    height: 280px;
    border: 5px solid green;
  }

  .flexbox > div {
    contain: size;
    width: 50%;
    height: 40px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
