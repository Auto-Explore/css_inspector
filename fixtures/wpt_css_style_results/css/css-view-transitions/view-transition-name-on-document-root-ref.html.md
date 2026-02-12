# css/css-view-transitions/view-transition-name-on-document-root-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-name-on-document-root-ref.html"
}
```

## style[0]

```css

html {
  view-transition-name: none;
}

:root {
  view-transition-name: root;
}

.foo {
  position: fixed;
  left: 0;
  top: 0;
  background: red;
  width: 100px;
  height: 100px;
  z-index: 1000;
}

.bar {
  position: fixed;
  left: 50px;
  top: 50px;
  background: green;
  width: 100px;
  height: 100px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
