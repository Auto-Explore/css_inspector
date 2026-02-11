# css/css-break/out-of-flow-in-multicolumn-017.html

```json
{
  "format_version": 3,
  "file": "css/css-break/out-of-flow-in-multicolumn-017.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 2;
    column-fill: auto;
    column-gap: 0px;
  }
  #outer {
    height: 100px;
    width: 300px;
  }
  #inner {
    width: 100px;
    background-color: red;
    position: relative;
    left: -150px;
  }
  .rel {
    position: relative;
    height: 360px;
  }
  .abs {
    position: absolute;
    height: 180px;
    width: 50px;
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
