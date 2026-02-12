# css/css-anchor-position/last-successful-change-fallbacks-position-area.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/last-successful-change-fallbacks-position-area.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 600px;
    height: 300px;
    background: teal;
  }
  #anchor {
    position: relative;
    top: 100px;
    left: 100px;
    width: 100px;
    height: 100px;
    background: red;
    anchor-name: --a;
  }
  #anchored {
    position-anchor: --a;
    position-try-fallbacks: right center;
    position: absolute;
    width: 200px;
    height: 100px;
    position-area: left center;
    background: lime;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
