# css/css-overflow/line-clamp/webkit-line-clamp-047.html

```json
{
  "format_version": 3,
  "file": "css/css-overflow/line-clamp/webkit-line-clamp-047.html"
}
```

## style[0]

```css

  .clamp {
    display: -webkit-box;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 5;
    /* no overflow: hidden */
    background-color: yellow;
    padding: 0.5em;
    font: 16px / 32px serif;
    white-space: pre;
  }
  .float {
    /* Floats create a parallel fragmentation context, and even if
     * -webkit-line-clamp caused a forced break in the main context, that should
     * not cause the float to fragment. */
    float: left;
    background-color: orange;
    padding: 0.5em;
    height: 100px;
    width: 100px;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
