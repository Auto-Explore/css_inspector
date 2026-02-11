# css/css-break/flexbox/multi-line-column-flex-fragmentation-056.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-056.html"
}
```

## style[0]

```css

  .multicol {
    position: relative;
    columns: 4;
    column-fill: auto;
    column-gap: 0px;
    width: 100px;
    height: 100px;
    background: green;
  }

  .flex {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    background: red;

  }

  .flex>div {
    height: 40px;
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
