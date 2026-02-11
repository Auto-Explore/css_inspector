# css/css-view-transitions/scoped/nested-scope-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/nested-scope-ref.html"
}
```

## style[0]

```css


.box { position: relative; contain: strict; }
#scopeA { background: #ccc;
          left: 0; top: 0; width: 300px; height: 300px; }
#partA { background: #4af;
         left: 30px; top: 30px; width: 240px; height: 240px; }
#scopeB { background: #eee;
          left: 30px; top: 30px; width: 180px; height: 180px; }
#partB { background: cyan;
         left: 30px; top: 30px; width: 120px; height: 120px; }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
