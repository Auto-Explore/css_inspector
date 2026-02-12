# css/css-flexbox/flex-child-percent-basis-resize-1.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flex-child-percent-basis-resize-1.html"
}
```

## style[0]

```css

  .flex-container {
    display: flex;
    flex-direction: row;
    width: 100px;
  }

  .flex-item {
    flex: 1 0 auto;
  }

  .scroll-outer {
    overflow:hidden;
    /* this combination is important */
    height: 100%;
    max-height: 100px;
  }

  .scroll-inner {
    overflow:scroll;
    height:100%;
  }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
