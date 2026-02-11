# css/css-view-transitions/shadow-part-with-class.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/shadow-part-with-class.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: red;
}

::part(party) {
  view-transition-name: party;
  view-transition-class: green;
}

:root { view-transition-name: none; }
html::view-transition-group(*) { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 0 }
html::view-transition-group(party) {
  position: absolute;
  width: 100px;
  height: 100px;
  background: yellow;
}
html::view-transition-group(party.red) {
  background: red;
}
html::view-transition-group(party.green) {
  background: green;
}
```

```json
{
  "errors": 10,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-class”.",
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
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

      div {
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
