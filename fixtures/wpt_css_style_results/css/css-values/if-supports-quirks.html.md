# css/css-values/if-supports-quirks.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-supports-quirks.html"
}
```

## style[0]

```css

    div {
      color: red;
      --x: if(supports(width: 30): true_value; else: false_value;)
    }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
