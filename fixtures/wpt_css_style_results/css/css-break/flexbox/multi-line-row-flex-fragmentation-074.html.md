# css/css-break/flexbox/multi-line-row-flex-fragmentation-074.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-074.html"
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
    flex-wrap: wrap;
    height: 280px;
    border: 5px solid green;
    align-content: space-between;
  }

  .flexbox > div {
    contain: size;
    width: 95%; /* one flex item per line */
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
