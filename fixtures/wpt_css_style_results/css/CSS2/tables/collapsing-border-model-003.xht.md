# css/CSS2/tables/collapsing-border-model-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/collapsing-border-model-003.xht"
}
```

## style[0]

```css

            table
            {
                background: orange;
                border-collapse: collapse;
            }
            tr
            {
                height: 1in;
            }
            td
            {
                border-top: 1in solid orange;
                padding: 0;
                width: 50px;
            }
            #div1
            {
                background: blue;
                height: 1.5in;
                width: 100px;
            }
        
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
