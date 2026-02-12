# css/css-anchor-position/anchor-scroll-update-006.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-006.html"
}
```

## style[0]

```css

  #cb {
    position: absolute;
    inset: 0;
  }
  #scroller {
    margin-top: 300px;
    overflow-y: scroll;
    height: 100px;
  }
  #spacer { height: 400px; }
  #spacer.changed { height: 300px; }
  #anchor { anchor-name: --a; }
  #anchored {
    position: absolute;
    width: 100px;
    height: 100px;
    background-color: green;
    top: anchor(top);
    left: 0;
    position-anchor: --a;
    position-visibility: always;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
