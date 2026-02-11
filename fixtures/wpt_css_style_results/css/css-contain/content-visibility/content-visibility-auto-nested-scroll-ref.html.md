# css/css-contain/content-visibility/content-visibility-auto-nested-scroll-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-auto-nested-scroll-ref.html"
}
```

## style[0]

```css

#outer {
  width: 400px;
  height: 400px;
  contain: layout paint size;
}

#inner {
  position: relative;
  top: 100px;
  width: 100px;
  height: 100px;
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
