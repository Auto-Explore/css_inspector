# css/CSS2/cascade/inherit-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/cascade/inherit-002.xht"
}
```

## style[0]

```css

        div
        {
            border: solid blue;
        }
        div p
        {
            border: inherit;
            border-color: orange;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “border-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
