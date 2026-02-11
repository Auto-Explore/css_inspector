# css/css-break/out-of-flow-in-multicolumn-004.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-004.html"
}
```

## style[0]

```css

  #multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0px;
    height: 40px;
    width: 100px;
  }
  .rel {
    position: relative;
  }
  .abs {
    position: absolute;
    width: 40px;
    height: 30px;
    background: green;
  }
  .content {
    width: 20px;
    height: 20px;
    background: blue;
  }
  .inside {
    break-inside: avoid;
  }
  .before {
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
