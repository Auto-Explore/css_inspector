# css/css-view-transitions/html-becomes-fixed.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/html-becomes-fixed.html"
}
```

## style[0]

```css


html { view-transition-name: none }
.f { position: fixed; background: #eee }
::view-transition-group(*) { animation-duration: 0s }
#part { position: absolute; left: 50px; top: 50px; width: 50px; height: 50px;
        padding: 5px; view-transition-name: p; background: #acf; }

```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
