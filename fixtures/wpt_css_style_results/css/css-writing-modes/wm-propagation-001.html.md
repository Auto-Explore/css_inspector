# css/css-writing-modes/wm-propagation-001.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/wm-propagation-001.html"
}
```

## style[0]

```css

html {
  writing-mode: horizontal-tb;
}
body {
  writing-mode: vertical-rl;
  width: 0; height: 0;
}
html::after {
  content: "This text must be horizontal.";
  display: block;
  /* The writing mode inherited from the root must be horizontal
  since the root's computed value should be horizontal */

}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
