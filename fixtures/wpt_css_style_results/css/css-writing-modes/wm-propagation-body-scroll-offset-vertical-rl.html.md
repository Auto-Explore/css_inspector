# css/css-writing-modes/wm-propagation-body-scroll-offset-vertical-rl.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-scroll-offset-vertical-rl.html"
}
```

## style[0]

```css

  html { writing-mode: vertical-lr; }
  body { writing-mode: vertical-rl; width: 10000px; }

  .result-wm, .result-wm > body {
    writing-mode: horizontal-tb;
    width: auto;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
