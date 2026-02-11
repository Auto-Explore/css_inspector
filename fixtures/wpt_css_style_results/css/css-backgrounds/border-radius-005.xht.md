# css/css-backgrounds/border-radius-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-005.xht"
}
```

## style[0]

```css

            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-radius: 50px 15px 40px / 20px 25px;
            }
            #reference {
                border-top-left-radius: 50px 20px;
                border-top-right-radius: 15px 25px;
                border-bottom-right-radius: 40px 20px;
                border-bottom-left-radius: 15px 25px;
            }
        
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    },
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
