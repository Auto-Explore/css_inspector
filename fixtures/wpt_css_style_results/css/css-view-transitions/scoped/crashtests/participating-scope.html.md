# css/css-view-transitions/scoped/crashtests/participating-scope.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/crashtests/participating-scope.html"
}
```

## style[0]

```css

body { margin: 5px; }
#scope { contain: strict; position: relative; z-index: 0;
         width: 150px; height: 100px; border: 5px solid #ccc;
         view-transition-name: container; }
.item { width: 6rem; height: 4rem; margin: 10px; background: #def;
        view-transition-name: a1; contain: strict; }
::view-transition-group(*),
::view-transition-old(*),
::view-transition-new(*) { animation: unset; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
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
