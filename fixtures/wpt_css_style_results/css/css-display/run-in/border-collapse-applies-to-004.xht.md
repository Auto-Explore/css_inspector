# css/css-display/run-in/border-collapse-applies-to-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/border-collapse-applies-to-004.xht"
}
```

## style[0]

```css

            div
            {
                border-collapse: collapse;
                display: run-in;
                height: 100px;
                width: 100px;
            }
            #top
            {
                border-bottom: 10px solid blue;
            }
            #bottom
            {
                border-top: 10px dotted orange;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
