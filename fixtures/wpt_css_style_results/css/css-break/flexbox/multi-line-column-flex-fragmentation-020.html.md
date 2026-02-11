# css/css-break/flexbox/multi-line-column-flex-fragmentation-020.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-020.html"
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
  }
  .abs {
    position: absolute;
    width: 5px;
    background: green;
  }
  #flex {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    height: 500px;
  }
  #flex > div {
    background: green;
    width: 5px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
