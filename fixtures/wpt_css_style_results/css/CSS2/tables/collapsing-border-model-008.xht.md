# css/CSS2/tables/collapsing-border-model-008.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/collapsing-border-model-008.xht"
}
```

## style[0]

```css

            table
            {
                background: blue;
                border-collapse: collapse;
            }
            col
            {
                width: 165px;
            }
            td, #div1
            {
                height: 120px;
            }
            td
            {
                width: 110px;
                border-right: 110px solid blue;
                padding: 0;
            }
            #div1
            {
                background: orange;
                width: 220px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
