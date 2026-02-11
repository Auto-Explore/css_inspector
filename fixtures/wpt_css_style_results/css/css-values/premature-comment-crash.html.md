# css/css-values/premature-comment-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-values/premature-comment-crash.html"
}
```

## style[0]

```css
#a{color:/*}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unclosed comment.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
