# css/CSS2/tables/border-conflict-resolution-006.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/tables/border-conflict-resolution-006.xht"
}
```

## style[0]

```css

            table
            {
                border-bottom: 8px dashed orange;
                border-left: 16px double purple;
                border-right: 12px dotted yellow;
                border-top: 4px solid blue;
            }
            td
            {
                width: 6em;
                height: 6em;
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
