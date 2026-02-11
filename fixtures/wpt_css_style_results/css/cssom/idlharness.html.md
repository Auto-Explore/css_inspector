# css/cssom/idlharness.html

```json
{
  "format_version": 3,
  "file": "css/cssom/idlharness.html"
}
```

## style[0]

```css

@import url("data:text/css,");
@namespace x "y";
@page { @top-left {} }
@media all {}
#test { color: green; }
```

```json
{
  "errors": 0,
  "messages": [
    {
      "message": "Imported style sheets are not checked.",
      "severity": "Warning"
    }
  ],
  "warnings": 1
}
```
