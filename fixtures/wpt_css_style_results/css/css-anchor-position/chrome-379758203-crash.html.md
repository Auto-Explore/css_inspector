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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
