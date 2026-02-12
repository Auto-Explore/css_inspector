# css/css-anchor-position/position-area-align-justify-wm-dir.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/position-area-align-justify-wm-dir.html"
}
```

## style[0]

```css

  /* A 300x300 container with a 100x100 centered anchor */
  #container {
    position: relative;
    width: 300px;
    height: 300px;
  }
  #anchor {
    position: absolute;
    top: 100px;
    left: 100px;
    width: 100px;
    height: 100px;
    anchor-name: --anchor;
  }
  #anchored {
    position: absolute;
    width: 10px;
    height: 10px;
    inset: 10px 15px 20px 25px;
    position-anchor: --anchor;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
