# css/css-conditional/container-queries/crashtests/chrome-bug-1505250-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/crashtests/chrome-bug-1505250-crash.html"
}
```

## style[0]

```css

  div::first-letter{
    color: green;
  }
  div:only-of-type {
    container-type: size;
    list-style: my-circle inside;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
