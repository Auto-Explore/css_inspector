# css/css-break/out-of-flow-in-multicolumn-005.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-005.html"
}
```

## style[0]

```css

  #multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 10px;
    height: 100px;
    width: 200px;
  }
  .rel {
    position: relative;
    height: 10px;
    width: 100px;
    background: green;
  }
  .abs {
    position: absolute;
    width: 40px;
    height: 30px;
    break-after: column;
  }
  .content {
    width: 100px;
    height: 45px;
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
