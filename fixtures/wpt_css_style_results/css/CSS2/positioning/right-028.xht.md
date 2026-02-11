# css/CSS2/positioning/right-028.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/right-028.xht"
}
```

## style[0]

```css

            div
            {
                direction: rtl;
                height: 1in;
                position: relative;
            }
            #div1
            {
                border-right: 5px solid orange;
            }
            div div
            {
                border-right: 5px solid blue;
                right: -0pc;
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
