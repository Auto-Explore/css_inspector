# css/css-scrollbars/invalid-needs-layout-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-scrollbars/invalid-needs-layout-crash.html"
}
```

## style[0]

```css

#target::-webkit-scrollbar { position: absolute; }
#target.crash::-webkit-scrollbar { right: 0 }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
