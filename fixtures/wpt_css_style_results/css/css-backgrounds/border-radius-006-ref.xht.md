# css/css-backgrounds/border-radius-006-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-006-ref.xht"
}
```

## style[0]

```css

            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-top-left-radius: 50px 20px;
                border-top-right-radius: 15px 25px;
                border-bottom-right-radius: 50px 10%;
                border-bottom-left-radius: 15px 25px;
            }
        
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-left-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
