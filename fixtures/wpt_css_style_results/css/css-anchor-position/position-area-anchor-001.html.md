# css/css-anchor-position/position-area-anchor-001.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-anchor-001.html"
}
```

## style[0]

```css

  .container {
    width: 100px; height: 100px;
    border: solid gray;
    margin: 6px;
    position: relative;
    anchor-scope: all;
    float: left;
  }

  .block {
    background: silver;
    height: 40px;
    width: 40px;
    margin: 10px;
  }

  .anchor {
    position: absolute;
    border: solid orange;
    margin: 25px;
  }
  .controls .anchor,
  .pull-up .anchor {
    width: 5px;
    height: 5px;
  }
  .push-down .anchor {
    width: 40px;
    height: 40px;
  }

  .anchored {
    border: solid blue;
    position: absolute;
    position-area: span-bottom right;
    inset: 0;
    place-self: stretch;
  }

  body > div { clear: both; }
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-area”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
