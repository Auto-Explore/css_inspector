# css/css-backgrounds/border-radius-007-ref.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-007-ref.xht"
}
```

## style[0]

```css

            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-top-left-radius: 50px 0.5in;
                border-top-right-radius: 10mm 25px;
                border-bottom-right-radius: 6pc 30%;
                border-bottom-left-radius: 15% 70pt;
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
