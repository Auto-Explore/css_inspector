# css/css-anchor-position/auto-margins-position-area-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/auto-margins-position-area-ref.html"
}
```

## style[0]

```css

  .container {
    position: absolute;
    width: 400px;
    height: 400px;
    margin: 0 auto;
    border: 2px solid;
  }
  .anchor {
    margin-top: 100px;
    margin-left: 100px;
    width: 150px;
    height: 75px;
    anchor-name: --anchor;
    border: solid 1px;
  }

  .anchored {
    position: absolute;
    position-anchor: --anchor;
    width: 20px;
    height: 20px;
    border: solid 1px;
    opacity: 30%;
    margin: 0;
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
