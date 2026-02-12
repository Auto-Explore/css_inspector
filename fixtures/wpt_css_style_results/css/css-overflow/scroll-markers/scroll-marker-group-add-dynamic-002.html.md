# css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-002.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/scroll-markers/scroll-marker-group-add-dynamic-002.html"
}
```

## style[0]

```css

  #scrollable::scroll-marker-group {
    display: block;
    height: 50px;
  }

  #scrollable.extra::scroll-marker-group {
    display: flex;
  }

  #scrollable>*::scroll-marker {
    display: block;
    width: 50px;
    height: 50px;
    content: "";
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
