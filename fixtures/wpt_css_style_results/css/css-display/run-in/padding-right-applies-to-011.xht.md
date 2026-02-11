# css/css-display/run-in/padding-right-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/padding-right-applies-to-011.xht"
}
```

## style[0]

```css

            div
            {
                height: 200px;
            }
            #div1
            {
                border-right: 10px solid blue;
                display: run-in;
                padding-right: 50px;
                width: 200px;
            }
            div div
            {
                border-right: 10px solid orange;
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
