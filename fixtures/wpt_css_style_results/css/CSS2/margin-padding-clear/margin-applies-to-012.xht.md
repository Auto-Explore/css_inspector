# css/CSS2/margin-padding-clear/margin-applies-to-012.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-applies-to-012.xht"
}
```

## style[0]

```css

            #wrapper
            {
                border: 10px solid blue;
                position: absolute;
            }
            #test
            {
                border: 10px solid orange;
                display: inline-block;
                height: 200px;
                width: 200px;
                margin: 50px;
                vertical-align: bottom;
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
