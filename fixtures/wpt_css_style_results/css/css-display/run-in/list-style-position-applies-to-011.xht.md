# css/css-display/run-in/list-style-position-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/list-style-position-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                display: run-in;
                list-style-position: inside;
                margin-left: 1in;
            }
            span
            {
                background: orange;
                display: list-item;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
