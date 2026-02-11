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
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “list-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
