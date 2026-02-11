# css/css-shadow/part/host-part-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/host-part-001.html"
}
```

## style[0]

```css

      :host::part(mypart) {
        color: lime;
      }
      :host(.tweak)::part(mypart) {
        color: blue;
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
