# css/css-display/display-contents-shadow-dom-1.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-shadow-dom-1.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:20px/1 monospace; padding:0; margin:0;
}
.before::before, ::slotted(.before)::before {content: "a ";}
.after::after, ::slotted(.after)::after {content: " c";}
div.before::before {content: "X ";}
div.after::after {content: " Y";}
.b, .c, ::slotted(.b), ::slotted(.c) { display:contents; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
