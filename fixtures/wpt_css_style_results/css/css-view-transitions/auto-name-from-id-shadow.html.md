# css/css-view-transitions/auto-name-from-id-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/auto-name-from-id-shadow.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
}

main {
  display: flex;
  flex-direction: column;
}

.item {
  view-transition-name: auto;
}

::part(p2) {
  view-transition-name: auto;
  background: yellow;
  position: relative;
  left: 100px;
  width: 100px;
  height: 100px;
}

main.switch #item1 {
  order: 2;
}

#item1 {
  background: green;
}

html::view-transition {
  background: rebeccapurple;
}

:root { view-transition-name: none; }
html::view-transition-group(*) {
  animation-timing-function: steps(2, start);
  animation-play-state: paused;
}
html::view-transition-group(item2) {
  outline: 10px solid red;
}

html::view-transition-old(*),
html::view-transition-new(*)
 { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 1 }

```

```json
{
  "errors": 11,
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
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
