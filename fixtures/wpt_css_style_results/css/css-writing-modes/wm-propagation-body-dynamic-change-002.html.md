# css/css-writing-modes/wm-propagation-body-dynamic-change-002.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-dynamic-change-002.html"
}
```

## style[0]

```css

  #new-body {
    /* This writing-mode should propagate to the root element. */
    writing-mode: vertical-rl;
    margin: 0;
  }

  #old-body {
    writing-mode: horizontal-tb;
    inline-size: 100px;
  }

  div {
    background-color: blue;
    height: 100px;
    width: 100px;
  }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
