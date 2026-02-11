# css/css-anchor-position/chrome-379758203-crash.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/chrome-379758203-crash.html"
}
```

## style[0]

```css

  .spacer { height: 2000px; }
  .anchor { anchor-name: --a; }
  #popover {
    transition: bottom 1s allow-discrete;
    position-anchor: --a;
    position-try-fallbacks: flip-block;
    top: anchor(bottom);
    bottom: auto;
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
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
