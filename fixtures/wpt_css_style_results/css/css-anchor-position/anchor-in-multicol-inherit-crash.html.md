# css/css-anchor-position/anchor-in-multicol-inherit-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-in-multicol-inherit-crash.html"
}
```

## style[0]

```css

.cb {
  position: relative;
  border: 1px solid black;
}
.columns {
  column-count: 4;
  column-width: 100px;
  width: 320px;
  height: 100px;
}
.anchor {
  anchor-name: --a1;
  width: 100px;
  height: 90px;
  background: blue;
}
.target {
  position: absolute;
  position-anchor: --a1;
  width: 10px;
  height: 10px;
  background: purple;
  position-try-fallbacks: --pf1, --pf2;
}
.child {
  position: fixed;
  right: inherit;
  bottom: inherit;
}
@position-try --pf1 {
  right: 400px;
  bottom: 10px;
}
@position-try --pf2 {
  right: 40px;
  bottom: 30px;
}
```

```json
{
  "errors": 4,
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
