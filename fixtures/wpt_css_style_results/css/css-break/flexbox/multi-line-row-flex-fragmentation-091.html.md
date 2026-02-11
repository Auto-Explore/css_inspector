# css/css-break/flexbox/multi-line-row-flex-fragmentation-091.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-row-flex-fragmentation-091.html"
}
```

## style[0]

```css

  .multicol {
    position: relative;
    columns: 2;
    column-fill: auto;
    width: 100px;
    height: 100px;
    background: green;
    column-gap: 0px;
  }
  .flex {
    display: flex;
    flex-wrap: wrap;
    background: red;
  }
  .flex>div {
    width: 25px;
    height: 50px;
    background: green;
  }
  .abspos {
    position: absolute;
    left: 0px;
    top: 50px;
    width: 50px;
    height: 50px;
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
