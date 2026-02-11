# css/css-anchor-position/anchor-scroll-position-try-003.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-003.html"
}
```

## style[0]

```css

#containing-block {
  width: 400px;
  height: 400px;
  transform: scale(1);
  border: 1px solid black;
}

#scroller {
  display: flex;
  flex-direction: column-reverse;
  width: 400px;
  height: 400px;
  overflow-y: scroll;
}

.box {
  min-height: 100px;
  width: 100px;
}

#anchor {
  anchor-name: --a;
  background: orange;
}

#anchored {
  position: absolute;
  top: anchor(bottom);
  width: 100px;
  height: 100px;
  background: green;
  position-anchor: --a;
  position-try-fallbacks: --pf;
}

@position-try --pf {
  top: auto;
  bottom: anchor(top);
}
```

```json
{
  "errors": 5,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
