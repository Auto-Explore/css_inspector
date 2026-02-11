# css/css-images/cross-fade-legacy-2-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-images/cross-fade-legacy-2-crash.html"
}
```

## style[0]

```css

      .test { background-image: -webkit-cross-fade(none, none, calc(13% + 1% * sign(1em - 1px))); }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-image”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
