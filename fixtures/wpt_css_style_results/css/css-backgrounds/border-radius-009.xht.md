# css/css-backgrounds/border-radius-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-009.xht"
}
```

## style[0]

```css

            body {
                border-radius: 20% 25px;
            }
            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-radius: inherit;
            }
            #reference {
                border-top-left-radius: 20%;
                border-top-right-radius: 25px;
                border-bottom-right-radius: 20%;
                border-bottom-left-radius: 25px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
