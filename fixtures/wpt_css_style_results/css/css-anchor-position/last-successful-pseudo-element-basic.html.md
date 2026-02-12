# css/css-anchor-position/last-successful-pseudo-element-basic.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/last-successful-pseudo-element-basic.html"
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
  #anchored::before {
    position-anchor: --a;
    /*
        We can't use position-area, since there's no way to access offsetTop of
        pseudo-elements to check for position. Instead, we have to use anchor()
        and check the computed inset values.
    */
    bottom: anchor(top);
    left: anchor(left);
    position-try-fallbacks: flip-block;
    position: absolute;
    width: 100px;
    height: 200px;
    background: lime;
    content: "";
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
