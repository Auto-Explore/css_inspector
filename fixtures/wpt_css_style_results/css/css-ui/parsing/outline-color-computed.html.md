# css/css-ui/parsing/outline-color-computed.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/parsing/outline-color-computed.html"
}
```

## style[0]

```css

  #target {
    color: blue;
    outline-style: auto;
    outline-color: auto;
  }
  #target2 {
    color: blue;
    outline-style: solid;
    outline-color: auto;
  }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “outline-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “outline-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
