# css/css-writing-modes/wm-propagation-body-049.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-049.html"
}
```

## style[0]

```css

  html
    {
      writing-mode: sideways-rl;
    }

  html::after
    {
      content: "This text must be vertical, with letters upright.";
      text-orientation: upright;
      /* 'text-orientation: upright' has no effect with 'sideways-rl', but does with 'vertical-rl' */
    }

  body
    {
      writing-mode: vertical-rl;
    }

  div
    {
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
