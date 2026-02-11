# css/css-display/run-in/padding-applies-to-011.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/padding-applies-to-011.xht"
}
```

## style[0]

```css

            #div1
            {
                border: 10px solid blue;
                display: run-in;
                padding: 50px;
                width: 200px;
            }
            div div
            {
                border: 10px solid orange;
                height: 200px;
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
