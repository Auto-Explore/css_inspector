# css/css-anchor-position/anchor-scroll-fixedpos-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-fixedpos-003.html"
}
```

## style[0]

```css

  body {
    margin: 0;
  }

  div {
    width: 100px;
    height: 100px;
    font-size: 16px;
  }

  #anchor {
    anchor-name: --a1;
    margin-top: 105vh;
    background: orange;
  }

  #anchored {
    position: fixed;
    position-anchor: --a1;
    left: anchor(left);
    bottom: anchor(top);
    background: green;
    color: white;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
