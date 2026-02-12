# css/css-flexbox/flexbox-safe-overflow-position-004.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-safe-overflow-position-004.html"
}
```

## style[0]

```css

  #reference-overlapped-red {
    position: absolute;
    background-color: red;
    width: 100px;
    height: 100px;
    z-index: -1;
  }

  .flex {
    display: flex;
    flex-flow: column-reverse;
    width: 90px;
    height: 90px;
    align-content: safe flex-start;
    justify-content: safe flex-start;
  }

  .item {
    flex: 0 0 100px;
    width: 100px;
    height: 100px;
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
