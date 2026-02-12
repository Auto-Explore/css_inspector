# css/css-view-transitions/small-scale-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/small-scale-ref.html"
}
```

## style[0]

```css

  :root { background: pink }
  body { margin: 0 }
  .outer {
    /* Removing this makes it work */
    transform: scale(0.5);
  }
  .inner {
    width: 200px;
    height: 200px;
    background-color: green;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
