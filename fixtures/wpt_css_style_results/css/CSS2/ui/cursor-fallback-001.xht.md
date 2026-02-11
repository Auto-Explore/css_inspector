# css/CSS2/ui/cursor-fallback-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/ui/cursor-fallback-001.xht"
}
```

## style[0]

```css

            div
            {
                background-color: blue;
                cursor: url("missing.cur"), pointer;
                border: solid 2px blue;
                width: 100px;
                height: 100px;
            }
        
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “cursor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
