# css/css-view-transitions/old-content-is-inline-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/old-content-is-inline-ref.html"
}
```

## style[0]

```css

:root {
  background-color: rebeccapurple;
  font: 20px/1 Ahem;
}

body { margin: 0; }

.container {
  position: absolute;
  left: 100px;
  width: 400px;
  height: 100px;
  background-color: grey;
}

.container.start {
  top: 100px;
}

.container.end {
  top: 300px;
}

.container.transitioned {
  left: 20px;
  width: 600px;
  transform: translateY(-50px);
}

.inline {
  background-color: limegreen;
  color: blue;
}

.transitioned .inline {
  opacity: 0;
}

#dummyEndInline {
  position: absolute;
  left: 20px;
  top: 250px;
  /* scale transform applied in script below */
  transform-origin: top left;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “transform-origin”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
