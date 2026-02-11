# css/css-cascade/scope-import-parent-pseudo.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-import-parent-pseudo.tentative.html"
}
```

## style[0]

```css

@import url("resources/scope-imported.css") scope((.scope));
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
