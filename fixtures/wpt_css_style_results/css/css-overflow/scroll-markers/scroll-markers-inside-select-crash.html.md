# css/css-overflow/scroll-markers/scroll-markers-inside-select-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-markers-inside-select-crash.html"
}
```

## style[0]

```css

  #scroller {
    overflow: scroll;
    scroll-marker-group: after;
  }
  #scroller div::scroll-marker {
    content: counter(test);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
