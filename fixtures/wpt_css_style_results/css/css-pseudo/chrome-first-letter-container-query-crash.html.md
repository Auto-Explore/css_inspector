# css/css-pseudo/chrome-first-letter-container-query-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/chrome-first-letter-container-query-crash.html"
}
```

## style[0]

```css

  head, html, body, span { display: inline-block; }
  html::first-letter, span::first-letter {
    color: pink;
  }
  span {
    container-type: inline-size;
    float: right;
  }
  body {
    writing-mode: vertical-rl;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
