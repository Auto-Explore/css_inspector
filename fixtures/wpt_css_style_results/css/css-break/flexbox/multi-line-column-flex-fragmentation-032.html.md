# css/css-break/flexbox/multi-line-column-flex-fragmentation-032.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-032.html"
}
```

## style[0]

```css

  .multicol {
    column-count: 5;
    column-fill: auto;
    column-gap: 0px;
    height: 100px;
    width: 100px;
    position: relative;
    background: red;
    z-index: -1;
  }
  #flex {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    height: 500px;
    position: relative;
  }
  #flex > div {
    background: green;
    width: 10px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
