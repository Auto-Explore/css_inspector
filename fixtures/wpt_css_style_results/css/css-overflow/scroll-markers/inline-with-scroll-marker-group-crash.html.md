# css/css-overflow/scroll-markers/inline-with-scroll-marker-group-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/inline-with-scroll-marker-group-crash.html"
}
```

## style[0]

```css

  #test {
    overflow: auto;
    scroll-marker-group: after;
  }
  #test::scroll-marker-group {
    display: block;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
