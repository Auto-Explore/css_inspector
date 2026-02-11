# css/css-scroll-anchoring/subtree-exclusion.html

```json
{
  "format_version": 3,
  "file": "css/css-scroll-anchoring/subtree-exclusion.html"
}
```

## style[0]

```css


body { height: 4000px }
#A, #B { width: 100px; background-color: #afa; }
#B { height: 100px; }
#inner { width: 100px; height: 100px; background-color: pink; }
#A { overflow-anchor: none; }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
