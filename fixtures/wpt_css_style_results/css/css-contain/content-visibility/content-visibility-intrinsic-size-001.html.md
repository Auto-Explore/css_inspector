# css/css-contain/content-visibility/content-visibility-intrinsic-size-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-intrinsic-size-001.html"
}
```

## style[0]

```css

    #container {
        position: sticky;
    }
    #container > div {
        font: 25px/1 Ahem;
        content-visibility: auto;
        contain-intrinsic-size: 1px 5000px;
    }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “contain-intrinsic-size”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
