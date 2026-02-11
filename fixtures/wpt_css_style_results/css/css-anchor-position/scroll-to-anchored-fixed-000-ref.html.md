# css/css-anchor-position/scroll-to-anchored-fixed-000-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/scroll-to-anchored-fixed-000-ref.html"
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

  /* Attach to anchor in both axes */
  .fixed {
    position: absolute;
    position-anchor: --foo;
    left: calc(100% - 4em);
    top: calc(100% - 4em);
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
  "errors": 3,
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
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
