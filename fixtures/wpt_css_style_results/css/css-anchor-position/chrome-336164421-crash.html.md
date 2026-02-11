# css/css-anchor-position/chrome-336164421-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-336164421-crash.html"
}
```

## style[0]

```css

  #inner {
    position: absolute;
    left: anchor(left);
  }
  #inner::before {
    display: none;
    content: "";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
