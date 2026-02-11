# css/css-contain/content-visibility/content-visibility-077.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-077.html"
}
```

## style[0]

```css

@keyframes cv {
  from { content-visibility: auto }
  to { content-visibility: hidden }
}

#container { animation: cv 1s; }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
