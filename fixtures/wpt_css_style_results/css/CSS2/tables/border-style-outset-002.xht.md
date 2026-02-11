# css/CSS2/tables/border-style-outset-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-style-outset-002.xht"
}
```

## style[0]

```css

            #table1
            {
                border-collapse: separate;
            }
            #table2
            {
                border: 30px outset orange;
                border-collapse: collapse;
            }
            td
            {
                height: 5em;
                width: 5em;
            }
            #table1 td
            {
                border: 30px groove orange;
            }
            #table2 td
            {
                border: 30px outset orange;
            }
        
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
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
