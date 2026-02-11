# css/css-view-transitions/multiline-span-with-overflowing-text-and-box-decorations-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/multiline-span-with-overflowing-text-and-box-decorations-ref.html"
}
```

## style[0]

```css

:root {
  font: 20px/1 Ahem;
}

div {
  width: 250px;
  visibility: hidden;
}
span {
  text-shadow: red -2px -5px;
  border: 2px solid black;
  box-shadow: 3px 3px red, -1em 0 .4em olive;
  view-transition-name: target;
  visibility: visible;
}

body {
  background: pink;
}
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
