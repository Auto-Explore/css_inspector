# css/css-pseudo/first-letter-bidi-pre-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/first-letter-bidi-pre-crash.html"
}
```

## style[0]

```css

    * {
        float: inline-end;
    }

    *::first-letter {
    }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
