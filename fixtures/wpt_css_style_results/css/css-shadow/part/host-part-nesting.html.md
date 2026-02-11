# css/css-shadow/part/host-part-nesting.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/part/host-part-nesting.html"
}
```

## style[0]

```css

      :host {
        &::part(mypart) {
          color: lime;
        }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
