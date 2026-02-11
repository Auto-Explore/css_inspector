# css/css-shadow/has-slotted-functional-changing-001.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/has-slotted-functional-changing-001.tentative.html"
}
```

## style[0]

```css

      p {
        color: rgb(0 255 0);
      }
      slot:not(:has-slotted) + p {
        color: rgb(0 0 255);
      }
      slot:not(:has-slotted(div)) + p {
        color: rgb(255 0 255);
      }
    
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
