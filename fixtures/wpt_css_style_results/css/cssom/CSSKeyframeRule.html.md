# css/cssom/CSSKeyframeRule.html

```json
{
  "format_version": 3,
  "file": "css/cssom/CSSKeyframeRule.html"
}
```

## style[0]

```css

  div { animation: 3s slidein; }
  @keyframes slidein {
    from {
      margin-left: 100%;
      width: 300%;
    }

    to {
      margin-left: 0%;
      width: 100%;
    }
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
