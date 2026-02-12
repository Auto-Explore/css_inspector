# css/css-anchor-position/anchor-scroll-fixedpos-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-fixedpos-004.html"
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
  }

  #anchor {
    anchor-name: --a1;
    margin-top: 105vh;
    background: orange;
  }

  #anchored {
    position: fixed;
    position-anchor: --a1;
    position-area: top right;
    background: green;
    color: white;

    transform: scale(2);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
