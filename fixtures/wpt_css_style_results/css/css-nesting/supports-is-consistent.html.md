# css/css-nesting/supports-is-consistent.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/supports-is-consistent.html"
}
```

## style[0]

```css

  /* This test is expected to pass even if the browser does not support nesting. */
  @supports selector(&) {
    p {
      color: red;
      & { color: inherit; }
    }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
