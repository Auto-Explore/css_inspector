# css/css-view-transitions/shadow-part-with-name-overridden-by-important.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/shadow-part-with-name-overridden-by-important.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  background: green;
  position: absolute;
}

::part(party) {
  view-transition-name: animate-me;
}

html {
  view-transition-name: none;
}

html::view-transition-group(*) { animation-play-state: paused; }
html::view-transition-old(*) { animation: unset; opacity: 0 }
html::view-transition-new(*) { animation: unset; opacity: 0 }
html::view-transition-group(animate-me) {
  position: absolute;
  width: 100px;
  height: 100px;
  background: red;
}
```

```json
{
  "errors": 7,
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

## style[1]

```css

      div {
        width: 100px;
        height: 100px;
        position: absolute;
        view-transition-name: please-dont-animate-me !important;
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
