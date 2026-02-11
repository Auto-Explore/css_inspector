# css/css-anchor-position/scroll-to-anchored-fixed-006-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/scroll-to-anchored-fixed-006-ref.html"
}
```

## style[0]

```css

  /* Force scrolling. */
  body {
    border: solid silver;
    height: 100vh;
    width: 100vw;
  }
  .anchor {
    position: absolute;
    top: 100%;
    left: 100%;
    width: 100vw;
    height: 100vh;
    border: solid silver;
    anchor-name: --foo;
  }

  /* Attach to anchor in horizontal axis */
  .fixed {
    position: absolute;
    position-anchor: --foo;
    position-area: span-all center;
    place-self: start;
    top: calc(100vh - 5em);
    padding: 1em 2em;
    border: solid orange 10px;
    margin: 5px;
  }

  /* Avoid pixel differences. */
  .fixed {
    font-family: Ahem;
  }
  a:focus {
    outline: solid blue;
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
