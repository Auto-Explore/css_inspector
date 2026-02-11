# css/css-writing-modes/wm-propagation-body-047.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-body-047.html"
}
```

## style[0]

```css

  html
    {
      writing-mode: vertical-lr;
    }

  body
    {
      writing-mode: sideways-lr;
    }

  html::after
    {
      content: "This text must be written sideways: vertically, with letters rotated 90 degrees.";
      text-orientation: upright; /* this has no effect with sideways-rl, but does with vertical-rl*/
    }

  div
    {
      background-color: teal;
      height: 100px;
      width: 100px;
    }
  img
    {
      display: block;
      margin-left: 1em;
      margin-right: 1em;
    }

  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
