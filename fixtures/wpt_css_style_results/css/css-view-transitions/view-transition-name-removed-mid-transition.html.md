# css/css-view-transitions/view-transition-name-removed-mid-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-name-removed-mid-transition.html"
}
```

## style[0]

```css

.target {
  view-transition-name:foo;
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
