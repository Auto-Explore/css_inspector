# css/css-lists/crashtests/chrome-bug-1377573.html

```json
{
  "format_version": 3,
  "file": "css/css-lists/crashtests/chrome-bug-1377573.html"
}
```

## style[0]

```css

  :is(body, html)::after {
    display: list-item;
    content: " ";
  }
  :root { list-style-image:url('.'); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
