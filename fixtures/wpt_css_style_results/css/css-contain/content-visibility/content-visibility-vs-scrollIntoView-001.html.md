# css/css-contain/content-visibility/content-visibility-vs-scrollIntoView-001.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-vs-scrollIntoView-001.html"
}
```

## style[0]

```css

    .auto {
        content-visibility: auto;
        contain-intrinsic-size: auto 1px auto 10000px;
    }

    .child {
        height: 40000px;
        position: relative;
    }

    #target {
        position: absolute;
        bottom: 0;
        font: 25px/1 Ahem;
    }

    .before_target {
        height: 40000px;
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
