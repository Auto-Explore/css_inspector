# css/css-break/flexbox/multi-line-column-flex-fragmentation-054.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-054.html"
}
```

## style[0]

```css

  .multicol {
    position: relative;
    columns: 2;
    column-fill: auto;
    width: 100px;
    column-gap: 0px;
    height: 100px;
    background: red;
  }

  .flex {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    background: red;
  }

  .flex>div {
    height: 100px;
    background: green;
  }

  .abspos {
    position: absolute;
    width: 50px;
    height: 10px;
    left: 50px;
    top: 90px;
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
