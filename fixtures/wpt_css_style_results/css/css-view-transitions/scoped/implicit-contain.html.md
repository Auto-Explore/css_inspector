# css/css-view-transitions/scoped/implicit-contain.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/implicit-contain.html"
}
```

## style[0]

```css


body { margin: 50px; }
.ib { display: inline-block; }
#scope {
  view-transition-name: foo;
  background: blue;
  width: 100px;
  height: 100px;
}
#scope::view-transition { background: red; }
#scope::view-transition-group(foo) { animation-play-state: paused; }
#scope::view-transition-new(foo) { animation: unset; opacity: 1; }
#scope::view-transition-old(foo) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
