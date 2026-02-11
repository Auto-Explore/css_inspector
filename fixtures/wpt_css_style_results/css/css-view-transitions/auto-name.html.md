# css/css-view-transitions/auto-name.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/auto-name.html"
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
  view-transition-class: item;
}

main.switch .item1 {
  order: 2;
}

.item1 {
  background: green;
}

.item2 {
  background: yellow;
  position: relative;
  left: 100px;
}

html::view-transition {
  background: rebeccapurple;
}

:root { view-transition-name: none; }
html::view-transition-group(.item) {
  animation-timing-function: steps(2, start);
  animation-play-state: paused;
}
html::view-transition-old(*),
html::view-transition-new(*)
 { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 1 }

```

```json
{
  "errors": 9,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-class”.",
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
    }
  ],
  "warnings": 0
}
```
