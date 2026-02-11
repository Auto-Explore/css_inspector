# css/css-backgrounds/border-radius-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-backgrounds/border-radius-003.xht"
}
```

## style[0]

```css

            div {
                border: 2px solid #a1a1a1;
                background: #dddddd;
                width: 200px;
                height: 100px;
                border-radius: 50px 0;
            }
            #reference {
                border-top-left-radius: 50px;
                border-top-right-radius: 0;
                border-bottom-right-radius: 50px;
                border-bottom-left-radius: 0;
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
