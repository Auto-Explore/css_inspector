# css/CSS2/syntax/unterminated-string-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/unterminated-string-001.xht"
}
```

## style[0]

```css

            div
            {
                color: green;
                font-family: 'Courier;
                color: red;
            }
        
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
