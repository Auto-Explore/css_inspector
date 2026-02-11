# css/css-contain/content-visibility/content-visibility-074-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-074-ref.html"
}
```

## style[0]

```css

#a { will-change: transform; }
#b { height: 15000px; }
#c {
  width: 800px;
  height: 600px;
}
#d {
  will-change: transform;
  top: 0px;
  width: 500px;
  height: 500px;
  background: green;
}
.contain {
  contain: layout style paint;
}

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “contain”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
