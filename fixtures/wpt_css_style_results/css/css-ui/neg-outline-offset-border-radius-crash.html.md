# css/css-ui/neg-outline-offset-border-radius-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/neg-outline-offset-border-radius-crash.html"
}
```

## style[0]

```css

#target {
  width: 100px;
  height: 10px;
  border-radius: 10px;
  outline: 20px solid green;
  outline-offset: -200px;
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
