# css/css-text/text-indent/text-indent-long-line-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-text/text-indent/text-indent-long-line-crash.html"
}
```

## style[0]

```css

body {
  width: 2147483648px;
}
div {
  text-indent: 2147483648px;
  width: 200%; /* Double the width in case CSS parser clamps the body width */
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
