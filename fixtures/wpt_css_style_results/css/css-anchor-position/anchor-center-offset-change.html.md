# css/css-anchor-position/anchor-center-offset-change.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-center-offset-change.html"
}
```

## style[0]

```css

  #cb {
    position: relative;
    width: 200px;
    height: 200px;
  }
  #anchor {
    width: 100px;
    height: 100px;
    anchor-name: --anchor;
  }
  #anchored {
    position: absolute;
    width: 100px;
    height: 100px;
    position-anchor: --anchor;
    align-self: anchor-center;
    left: anchor(--unknown right, 0px);
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
