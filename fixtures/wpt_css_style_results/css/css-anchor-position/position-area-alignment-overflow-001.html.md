# css/css-anchor-position/position-area-alignment-overflow-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-alignment-overflow-001.html"
}
```

## style[0]

```css

  .container {
    position: relative;
    overflow: scroll;
    scrollbar-width: none;
    width: 100px;
    height: 120px;
    border: solid;
    margin: 1em;
    float: left;
  }
  .anchor {
    border: solid blue 10px;
    anchor-name: --foo;
    inset: 0;
    place-self: center;
    position: absolute;
  }
  .test {
    border: solid 5px #0084;
    position: absolute;
    position-anchor: --foo;
    width: 100%;
    height: 100%;
  }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
