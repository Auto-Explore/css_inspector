# css/css-break/flexbox/multi-line-column-flex-fragmentation-040.html

```json
{
  "format_version": 3,
  "file": "css/css-break/flexbox/multi-line-column-flex-fragmentation-040.html"
}
```

## style[0]

```css

  #flex {
    display: flex;
    flex-direction: column;
    flex-wrap: wrap;
    height: 500px;
  }
  #flex > div {
    height: 100px;
    width: 10px;
  }
  #flex > div:nth-child(odd) {
    background: green;
    margin-top: -100px;
    break-before: column;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
