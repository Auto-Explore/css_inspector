# css/css-align/blocks/align-content-block-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-align/blocks/align-content-block-001-ref.html"
}
```

## style[0]

```css

  html, body { margin: 0; font-size: 10px; line-height: 1; color: transparent; }
  /* show bounds of test box without interfering with margin-collapsing */
    .test { background: black; padding-right: 2px; margin: 0.5em; }
  /* ensure float cannot penetrate */
    .outer-float { float: left; height: 600px; /* reftest limit = 600px */
                   margin: 0 1em; background: gray; }
  /* ensure float is contained */
    .float { float: left; background: orange; height: 2em }
  /* ensure margin is contained */
    .in-flow { margin-top: 1em; background: orange }

  /* reference code */
    .test { overflow: hidden; display: flow-root; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
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
