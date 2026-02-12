# css/css-overflow/scroll-markers/column-scroll-marker-no-content-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/column-scroll-marker-no-content-crash.html"
}
```

## style[0]

```css

  #container {
    overflow: hidden;
    width: 10px;
    column-count: 1;
    scroll-marker-group: before;
  }
  #container::column::scroll-marker {
    display: inline;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
