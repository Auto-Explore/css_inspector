# css/css-break/flexbox/single-line-column-flex-fragmentation-064.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/single-line-column-flex-fragmentation-064.html"
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
    flex-direction: column;
    height: 280px;
    justify-content: space-between;
    border: 5px solid green;
  }

  .flexbox > div {
    contain: size;
    width: 15px;
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
