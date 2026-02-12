# css/css-break/transform-007-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-break/transform-007-ref.html"
}
```

## style[0]

```css

  .transformed {
      will-change: transform;
      transform: rotate(25deg);
      transform-origin: bottom right;
      background: hotpink;
  }
  .child {
      margin-left: auto;
      width: 50px;
      background: lime;
  }
  .fake-column {
      float: left;
      width: 100px;
      height: 100px;
  }
  .fake-column:not(:last-child) {
      margin-right: 20px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
