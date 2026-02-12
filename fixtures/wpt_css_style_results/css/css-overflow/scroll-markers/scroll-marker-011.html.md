# css/css-overflow/scroll-markers/scroll-marker-011.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-011.html"
}
```

## style[0]

```css

  * {
    font-family: monospace;
  }

  .scroller {
    overflow: hidden;
    scroll-marker-group: before;
  }

  .scroller::scroll-marker-group {
    display: block;
    height: 2em;
    overflow: hidden;
  }

  .scroller>*::scroll-marker {
    content: attr(text);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
