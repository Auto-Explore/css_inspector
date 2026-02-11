# css/css-view-transitions/mix-blend-mode-only-on-transition.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/mix-blend-mode-only-on-transition.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: blue;
  contain: paint;
}
.tagone { view-transition-name: one }
.tagtwo { view-transition-name: two }
.tagthree { view-transition-name: three }
.tagfour { view-transition-name: four }
.tagfive { view-transition-name: five }

::view-transition-old(four) {
  animation-name: unset;
}
::view-transition-new(five) {
  animation-name: unset;
}
```

```json
{
  "errors": 7,
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
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
