# css/css-backgrounds/border-radius-010-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-010-ref.xht"
}
```

## style[0]

```css

            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-top-left-radius: 20% 25px;
                border-top-right-radius: 0;
                border-bottom-right-radius: 20pt 3em;
                border-bottom-left-radius: 0;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
