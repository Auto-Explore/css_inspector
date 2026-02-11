# css/css-anchor-position/anchor-scroll-update-005.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-update-005.html"
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
  #scroller.changed { height: 200px; }
  #spacer { height: 400px; }
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
  "errors": 3,
  "messages": [
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
