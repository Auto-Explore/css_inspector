# css/css-view-transitions/content-with-transparent-background-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/content-with-transparent-background-ref.html"
}
```

## style[0]

```css

.target {
  width: 100px;
  height: 100px;
  view-transition-name: target;
  position: fixed;
  top: 25px;
  left: 25px;
  background: rgba(0,0,0,0);
}

.embedded {
  width: 25%;
  height: 50%;
  position: absolute;
  top: 0px;
  will-change: opacity;
}

#one{
  left: 25%;
  background: lightblue;
}
#two {
  left: 50%;
  background: lightgreen;
}

html::view-transition-group(hidden) { animation-duration: 300s; }
html::view-transition-image-pair(hidden) { animation: unset; opacity: 0; }

html::view-transition-group(target) { animation: unset; }
html::view-transition-old(target) { animation: unset; opacity: 1; }
html::view-transition-new(target) { animation: unset; opacity: 0; }

```

```json
{
  "errors": 8,
  "messages": [
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
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
