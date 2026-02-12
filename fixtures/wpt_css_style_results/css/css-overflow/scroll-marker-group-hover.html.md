# css/css-overflow/scroll-marker-group-hover.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-marker-group-hover.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  #scroller {
    overflow: auto;
    scroll-marker-group: before;

    &::scroll-marker-group {
      background: red;
      height: 100px;
      width: 100px;
    }

    &::scroll-marker-group:hover {
      background: green;
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
