# css/css-anchor-position/last-successful-change-try-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/last-successful-change-try-rule.html"
}
```

## style[0]

```css

  #container {
    position: relative;
    width: 400px;
    height: 400px;
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
    position-try-fallbacks: --try;
    position: absolute;
    width: 100px;
    height: 200px;
    position-area: top center;
    background: lime;
  }
  @position-try --try {
    position-area: bottom center;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
