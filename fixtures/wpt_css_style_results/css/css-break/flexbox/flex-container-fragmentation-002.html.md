# css/css-break/flexbox/flex-container-fragmentation-002.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/flex-container-fragmentation-002.html"
}
```

## style[0]

```css

  .multicol {
    background: red;
    column-count: 4;
    column-gap: 0px;
    height: 100px;
    position: relative;
    width: 100px;
  }
  .flex {
    background: green;
    break-inside: avoid;
    display: flex;
    width: 25px;
  }
  .abs {
    background: green;
    height: 50px;
    position: absolute;
    width: 25px;
    top: 50px;
    left: 50px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
