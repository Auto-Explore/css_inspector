# css/css-break/flexbox/flex-container-fragmentation-010.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/flex-container-fragmentation-010.html"
}
```

## style[0]

```css

  .multicol {
    background: red;
    column-count: 4;
    column-gap: 0px;
    height: 100px;
    width: 100px;
    position: relative;
  }
  .flex {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    height: 400px;
    width: 50px;
  }
  .flex > div {
    background: green;
    height: 100px;
    width: 25px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
