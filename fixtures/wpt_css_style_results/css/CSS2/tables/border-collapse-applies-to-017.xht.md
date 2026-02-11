# css/CSS2/tables/border-collapse-applies-to-017.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-collapse-applies-to-017.xht"
}
```

## style[0]

```css

            #container
            {
                display: block;
            }
            .test
            {
                border-collapse: collapse;
                display: inherit;
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
