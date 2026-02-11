# css/css-anchor-position/pseudo-element-with-slotted-anchor-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/pseudo-element-with-slotted-anchor-ref.html"
}
```

## style[0]

```css

my-anchor {
  display: block;
}

span {
  display: block;
  anchor-name: --anchor;
  padding: 2rem;
  background: red;
  color: white;
  inline-size: fit-content;
}

my-anchor::after {
  content: 'target';
  position: absolute;
  position-anchor: --anchor;
  position-area: bottom center;
  background: lightblue;
  padding: 1rem;
}
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
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
