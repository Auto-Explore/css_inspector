# css/css-break/flexbox/multi-line-column-flex-fragmentation-055.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-055.html"
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
    height: 95px;
    background: green;
  }

  .abspos {
    position: absolute;
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
