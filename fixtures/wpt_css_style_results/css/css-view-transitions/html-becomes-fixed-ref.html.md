# css/css-view-transitions/html-becomes-fixed-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/html-becomes-fixed-ref.html"
}
```

## style[0]

```css


.f { position: fixed; background: #eee }
#part { position: absolute; left: 50px; top: 50px; width: 50px; height: 50px;
        padding: 5px; view-transition-name: p; background: #acf; }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
