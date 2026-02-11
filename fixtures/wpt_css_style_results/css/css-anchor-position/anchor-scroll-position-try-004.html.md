# css/css-anchor-position/anchor-scroll-position-try-004.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-004.html"
}
```

## style[0]

```css

body {
  margin: 0;
}

#vrl-scroller {
  writing-mode: vertical-rl;
  overflow-x: scroll;
  width: calc(100vw - 90px);
  height: 400px;
  outline: 1px solid black;
}

#anchor {
  anchor-name: --a;
  width: 100px;
  height: 100px;
  margin-left: 120lvw;
  background: orange;
}

#anchored {
  position: fixed;
  width: 100px;
  height: 100px;
  background: green;
  position-anchor: --a;
  position-try-fallbacks: --pf1, --pf2;
  /* Top of the anchor */
  bottom: anchor(top);
  left: anchor(left);
}

@position-try --pf1 {
  /* Bottom of the anchor */
  top: anchor(bottom);
  bottom: auto;
}
@position-try --pf2 {
  /* Left of the anchor */
  top: anchor(top);
  right: anchor(left);
  bottom: auto;
  left: auto;
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
