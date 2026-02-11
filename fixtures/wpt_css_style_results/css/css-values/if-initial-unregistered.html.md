# css/css-values/if-initial-unregistered.html

```json
{
  "format_version": 3,
  "file": "css/css-values/if-initial-unregistered.html"
}
```

## style[0]

```css

  div {
    color: if(style(--x:initial): green; else: red);
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “else”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
