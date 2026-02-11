# css/css-break/out-of-flow-in-multicolumn-007.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-007.html"
}
```

## style[0]

```css

  #multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0px;
    height: 100px;
    width: 100px;
    /* Move the multicolumn left to account for the first empty column */
    position: relative;
    left: -50px;
  }
  .rel {
    position: relative;
  }
  .abs {
    position: absolute;
    width: 50px;
    height: 200px;
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
